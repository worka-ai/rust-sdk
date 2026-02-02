use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse2, Error, FnArg, ItemFn, PatType, Result, ReturnType, Type, TypePath, TypeReference,
    TypeSlice,
};

pub fn worka(attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new_spanned(attr, "worka attribute does not accept arguments"));
    }

    let dispatch_fn: ItemFn = parse2(input)?;
    validate_dispatch_signature(&dispatch_fn)?;

    let dispatch_ident = &dispatch_fn.sig.ident;

    Ok(quote! {
        #dispatch_fn

        #[unsafe(no_mangle)]
        pub extern "C" fn worka_init() {}

        #[unsafe(no_mangle)]
        pub extern "C" fn worka_handle_request(ptr: u32, len: u32) -> u64 {
            let req_bytes = unsafe { __worka_slice_from_raw(ptr, len) };
            let envelope = <::runtime_proto::runtime::AbiEnvelope as ::prost::Message>::decode(req_bytes)
                .unwrap();
            let req = <::runtime_proto::runtime::AbiRequest as ::prost::Message>::decode(envelope.payload.as_slice())
                .unwrap();

            let mut response = ::runtime_proto::runtime::AbiResponse {
                request_id: req.request_id,
                payload: ::std::vec::Vec::new(),
                error: ::std::string::String::new(),
                outbound_calls: ::std::vec::Vec::new(),
            };

            #dispatch_ident(req.method.as_str(), &req.payload, &mut response);

            let mut buf = ::std::vec::Vec::new();
            <::runtime_proto::runtime::AbiResponse as ::prost::Message>::encode(&response, &mut buf).unwrap();
            let out_ptr = worka_alloc(buf.len() as u32);
            unsafe {
                let out = __worka_slice_from_raw_mut(out_ptr, buf.len() as u32);
                out.copy_from_slice(&buf);
            }

            __worka_pack_ptr_len(out_ptr, buf.len() as u32)
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn worka_alloc(len: u32) -> u32 {
            let mut buf = ::std::vec::Vec::<u8>::with_capacity(len as usize);
            let ptr = buf.as_mut_ptr();
            ::core::mem::forget(buf);
            ptr as u32
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn worka_free(ptr: u32, len: u32) {
            unsafe {
                let _ = ::std::vec::Vec::from_raw_parts(ptr as *mut u8, 0, len as usize);
            }
        }

        fn __worka_pack_ptr_len(ptr: u32, len: u32) -> u64 {
            ((len as u64) << 32) | (ptr as u64)
        }

        unsafe fn __worka_slice_from_raw(ptr: u32, len: u32) -> &'static [u8] {
            unsafe { ::core::slice::from_raw_parts(ptr as *const u8, len as usize) }
        }

        unsafe fn __worka_slice_from_raw_mut(ptr: u32, len: u32) -> &'static mut [u8] {
            unsafe { ::core::slice::from_raw_parts_mut(ptr as *mut u8, len as usize) }
        }
    })
}

fn validate_dispatch_signature(dispatch_fn: &ItemFn) -> Result<()> {
    if !dispatch_fn.sig.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &dispatch_fn.sig.generics,
            "worka dispatch function must not be generic",
        ));
    }

    if dispatch_fn.sig.asyncness.is_some() {
        return Err(Error::new_spanned(
            &dispatch_fn.sig.asyncness,
            "worka dispatch function must be synchronous",
        ));
    }

    match &dispatch_fn.sig.output {
        ReturnType::Default => {}
        ReturnType::Type(_, typ) => {
            if !matches!(&**typ, Type::Tuple(t) if t.elems.is_empty()) {
                return Err(Error::new_spanned(
                    typ,
                    "worka dispatch function must return ()",
                ));
            }
        }
    }

    if dispatch_fn.sig.inputs.len() != 3 {
        return Err(Error::new_spanned(
            &dispatch_fn.sig.inputs,
            "worka dispatch function must take exactly 3 arguments: (method: &str, payload: &[u8], response: &mut AbiResponse)",
        ));
    }

    let mut iter = dispatch_fn.sig.inputs.iter();
    validate_method_arg(iter.next().unwrap())?;
    validate_payload_arg(iter.next().unwrap())?;
    validate_response_arg(iter.next().unwrap())?;

    Ok(())
}

fn validate_method_arg(arg: &FnArg) -> Result<()> {
    let FnArg::Typed(PatType { ty, .. }) = arg else {
        return Err(Error::new_spanned(arg, "method must be a typed argument"));
    };
    let Type::Reference(TypeReference { mutability: None, elem, .. }) = &**ty else {
        return Err(Error::new_spanned(ty, "method must be of type &str"));
    };
    let Type::Path(TypePath { path, .. }) = &**elem else {
        return Err(Error::new_spanned(elem, "method must be of type &str"));
    };
    let Some(seg) = path.segments.last() else {
        return Err(Error::new_spanned(path, "method must be of type &str"));
    };
    if seg.ident != "str" {
        return Err(Error::new_spanned(seg, "method must be of type &str"));
    }
    Ok(())
}

fn validate_payload_arg(arg: &FnArg) -> Result<()> {
    let FnArg::Typed(PatType { ty, .. }) = arg else {
        return Err(Error::new_spanned(arg, "payload must be a typed argument"));
    };
    let Type::Reference(TypeReference { mutability: None, elem, .. }) = &**ty else {
        return Err(Error::new_spanned(ty, "payload must be of type &[u8]"));
    };
    let Type::Slice(TypeSlice { elem, .. }) = &**elem else {
        return Err(Error::new_spanned(elem, "payload must be of type &[u8]"));
    };
    let Type::Path(TypePath { path, .. }) = &**elem else {
        return Err(Error::new_spanned(elem, "payload must be of type &[u8]"));
    };
    let Some(seg) = path.segments.last() else {
        return Err(Error::new_spanned(path, "payload must be of type &[u8]"));
    };
    if seg.ident != "u8" {
        return Err(Error::new_spanned(seg, "payload must be of type &[u8]"));
    }
    Ok(())
}

fn validate_response_arg(arg: &FnArg) -> Result<()> {
    let FnArg::Typed(PatType { ty, .. }) = arg else {
        return Err(Error::new_spanned(arg, "response must be a typed argument"));
    };
    let Type::Reference(TypeReference { mutability: Some(_), elem, .. }) = &**ty else {
        return Err(Error::new_spanned(ty, "response must be of type &mut AbiResponse"));
    };
    let Type::Path(TypePath { path, .. }) = &**elem else {
        return Err(Error::new_spanned(elem, "response must be of type &mut AbiResponse"));
    };
    let Some(seg) = path.segments.last() else {
        return Err(Error::new_spanned(path, "response must be of type &mut AbiResponse"));
    };
    if seg.ident != "AbiResponse" {
        return Err(Error::new_spanned(
            seg,
            "response must be of type &mut runtime_proto::runtime::AbiResponse",
        ));
    }
    Ok(())
}

