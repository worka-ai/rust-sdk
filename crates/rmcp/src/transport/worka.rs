use std::future::Future;

use anyhow::{Result, anyhow};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
    sync::mpsc,
};
use tokio_util::{
    bytes::{BufMut, BytesMut},
    codec::{Decoder, Encoder, FramedRead, FramedWrite},
};

use crate::{model::JsonRpcMessage, service::ServiceRole, transport::Transport};

pub struct WorkaTransport<R: ServiceRole> {
    reader: FramedRead<
        tokio::net::unix::OwnedReadHalf,
        WorkaCodec<JsonRpcMessage<R::PeerReq, R::PeerResp, R::PeerNot>>,
    >,
    tx: mpsc::Sender<JsonRpcMessage<R::Req, R::Resp, R::Not>>,
}

impl<R: ServiceRole> WorkaTransport<R> {
    pub async fn connect(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let mut stream = UnixStream::connect(path).await?;
        register_pack_session_if_configured(&mut stream).await?;
        let (read_half, write_half) = stream.into_split();

        let (tx, mut rx) = mpsc::channel::<JsonRpcMessage<R::Req, R::Resp, R::Not>>(64);

        tokio::spawn(async move {
            let mut writer = FramedWrite::new(write_half, WorkaCodec::default());
            while let Some(msg) = rx.recv().await {
                if let Err(e) = writer.send(msg).await {
                    tracing::error!("WorkaTransport writer error: {}", e);
                    break;
                }
            }
            let _ = writer.get_mut().shutdown().await;
        });

        Ok(Self {
            reader: FramedRead::new(read_half, WorkaCodec::default()),
            tx,
        })
    }
}

async fn register_pack_session_if_configured(stream: &mut UnixStream) -> Result<()> {
    let Ok(session_id) = std::env::var("WORKA_PACK_SESSION_ID") else {
        return Ok(());
    };
    let request = WorkaSocketRequest {
        invocation_id: format!("register:{session_id}"),
        parent_invocation_id: None,
        ucan: std::env::var("WORKA_PACK_BOOTSTRAP_UCAN").unwrap_or_default(),
        cap: None,
        op: "worka.register_pack_session".to_string(),
        args: serde_json::json!({
            "session_id": session_id,
            "pack_tenant": std::env::var("WORKA_PACK_TENANT").unwrap_or_default(),
            "pack_name": std::env::var("WORKA_PACK_NAME").unwrap_or_default(),
        }),
    };
    stream.write_all(&serde_json::to_vec(&request)?).await?;
    stream.write_all(b"\n").await?;
    stream.flush().await?;
    Ok(())
}

impl<R: ServiceRole> Transport<R> for WorkaTransport<R> {
    type Error = std::io::Error;

    fn send(
        &mut self,
        message: JsonRpcMessage<R::Req, R::Resp, R::Not>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send + 'static {
        let tx = self.tx.clone();
        async move {
            tx.send(message)
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        }
    }

    fn receive(
        &mut self,
    ) -> impl Future<Output = Option<JsonRpcMessage<R::PeerReq, R::PeerResp, R::PeerNot>>> + Send
    {
        let reader = &mut self.reader;
        async move { reader.next().await.and_then(|r| r.ok()) }
    }

    fn close(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        let _ = self.tx.clone();
        async move { Ok(()) }
    }
}

pub struct WorkaCodec<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for WorkaCodec<T> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: for<'de> Deserialize<'de>> Decoder for WorkaCodec<T> {
    type Item = T;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(pos) = src.iter().position(|&b| b == b'\n') {
            let line = src.split_to(pos + 1);
            let value: JsonValue = serde_json::from_slice(&line[..pos])
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            let item_value = match value.get("op").and_then(|op| op.as_str()) {
                Some("mcp") => value.get("args").cloned().ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Worka mcp frame missing args",
                    )
                })?,
                Some(other) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("unsupported Worka transport op `{other}`"),
                    ));
                }
                None => value,
            };
            let item = serde_json::from_value(item_value)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}

impl<T: Serialize> Encoder<T> for WorkaCodec<T> {
    type Error = std::io::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let value = serde_json::to_value(&item)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let response = WorkaSocketResponse {
            invocation_id: Some(mcp_invocation_id(&value)),
            ok: true,
            value,
            error: None,
        };
        let json = serde_json::to_vec(&response)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        dst.extend_from_slice(&json);
        dst.put_u8(b'\n');
        Ok(())
    }
}

fn mcp_invocation_id(value: &JsonValue) -> String {
    match value.get("id") {
        Some(JsonValue::String(id)) => id.clone(),
        Some(JsonValue::Number(id)) => format!("mcp-{id}"),
        Some(other) => format!("mcp-{}", stable_json_fragment(other)),
        None => {
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or_default();
            format!("mcp-notification-{nanos}")
        }
    }
}

fn stable_json_fragment(value: &JsonValue) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|_| "unknown".to_string())
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}

#[derive(Debug, Clone)]
pub struct WorkaClient {
    socket_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkaInvocationMeta {
    pub invocation_id: String,
    pub ucan: String,
}

impl WorkaInvocationMeta {
    pub fn from_meta(meta: &crate::model::Meta) -> Result<Self> {
        let invocation_id = meta
            .get("invocation_id")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| anyhow!("Worka MCP request metadata is missing invocation_id"))?;
        let ucan = meta
            .get("ucan")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| anyhow!("Worka MCP request metadata is missing ucan"))?;

        Ok(Self {
            invocation_id: invocation_id.to_string(),
            ucan: ucan.to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkaSocketRequest {
    pub invocation_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_invocation_id: Option<String>,
    pub ucan: String,
    pub cap: Option<String>,
    pub op: String,
    pub args: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkaSocketResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    pub ok: bool,
    pub value: JsonValue,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Trace,
}

impl WorkaClient {
    pub fn new() -> Self {
        let path = std::env::var("WORKA_BROKER_SOCKET")
            .unwrap_or_else(|_| "/run/worka/broker.sock".to_string());
        Self { socket_path: path }
    }

    pub async fn http_request(
        &self,
        invocation_id: &str,
        ucan: &str,
        method: HttpMethod,
        url: &str,
        headers: Option<serde_json::Map<String, JsonValue>>,
        body: Option<JsonValue>,
    ) -> Result<JsonValue> {
        self.http_request_with_parent(invocation_id, None, ucan, method, url, headers, body)
            .await
    }

    pub async fn http_request_for_invocation(
        &self,
        invocation: &WorkaInvocationMeta,
        child_invocation_id: &str,
        method: HttpMethod,
        url: &str,
        headers: Option<serde_json::Map<String, JsonValue>>,
        body: Option<JsonValue>,
    ) -> Result<JsonValue> {
        self.http_request_with_parent(
            child_invocation_id,
            Some(&invocation.invocation_id),
            &invocation.ucan,
            method,
            url,
            headers,
            body,
        )
        .await
    }

    pub async fn http_request_with_parent(
        &self,
        invocation_id: &str,
        parent_invocation_id: Option<&str>,
        ucan: &str,
        method: HttpMethod,
        url: &str,
        headers: Option<serde_json::Map<String, JsonValue>>,
        body: Option<JsonValue>,
    ) -> Result<JsonValue> {
        let headers = headers.unwrap_or_default();
        let req = WorkaSocketRequest {
            invocation_id: invocation_id.to_string(),
            parent_invocation_id: parent_invocation_id.map(str::to_string),
            ucan: ucan.to_string(),
            cap: None,
            op: "http.request".to_string(),
            args: serde_json::json!({
                "method": method,
                "url": url,
                "headers": headers,
                "body": body,
            }),
        };

        self.send_request(req).await
    }

    pub async fn send_request(&self, req: WorkaSocketRequest) -> Result<JsonValue> {
        if self.socket_path.contains(':') {
            // TCP
            let stream = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                tokio::net::TcpStream::connect(&self.socket_path),
            )
            .await
            .map_err(|_| anyhow!("Worka connect timeout (TCP)"))??;
            let mut stream = stream;
            stream.write_all(&serde_json::to_vec(&req)?).await?;
            stream.write_all(b"\n").await?;

            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            reader.read_line(&mut line).await?;

            let res: WorkaSocketResponse = serde_json::from_str(&line)?;
            if res.invocation_id.as_deref() != Some(req.invocation_id.as_str()) {
                return Err(anyhow!(
                    "Worka response invocation_id mismatch: expected {}, got {:?}",
                    req.invocation_id,
                    res.invocation_id
                ));
            }
            if res.ok {
                Ok(res.value)
            } else {
                Err(anyhow!(
                    res.error
                        .unwrap_or_else(|| "Unknown worka error".to_string())
                ))
            }
        } else {
            // Unix
            let mut stream = UnixStream::connect(&self.socket_path).await?;
            stream.write_all(&serde_json::to_vec(&req)?).await?;
            stream.write_all(b"\n").await?;

            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            reader.read_line(&mut line).await?;

            let res: WorkaSocketResponse = serde_json::from_str(&line)?;
            if res.invocation_id.as_deref() != Some(req.invocation_id.as_str()) {
                return Err(anyhow!(
                    "Worka response invocation_id mismatch: expected {}, got {:?}",
                    req.invocation_id,
                    res.invocation_id
                ));
            }
            if res.ok {
                Ok(res.value)
            } else {
                Err(anyhow!(
                    res.error
                        .unwrap_or_else(|| "Unknown worka error".to_string())
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codec_decodes_mcp_envelope_args() {
        let mut codec = WorkaCodec::<JsonValue>::default();
        let mut bytes = BytesMut::from(
            br#"{"invocation_id":"mcp-1","ucan":"u","op":"mcp","args":{"jsonrpc":"2.0","id":1,"method":"tools/call"}}"#
                .as_slice(),
        );
        bytes.put_u8(b'\n');

        let decoded = codec.decode(&mut bytes).unwrap().unwrap();

        assert_eq!(decoded["method"], "tools/call");
        assert_eq!(decoded["id"], 1);
    }

    #[test]
    fn codec_encodes_json_rpc_as_worka_response() {
        let mut codec = WorkaCodec::<JsonValue>::default();
        let mut bytes = BytesMut::new();

        codec
            .encode(
                serde_json::json!({"jsonrpc": "2.0", "id": 7, "result": {"ok": true}}),
                &mut bytes,
            )
            .unwrap();
        let response: WorkaSocketResponse =
            serde_json::from_slice(&bytes[..bytes.len() - 1]).expect("encoded response");

        assert_eq!(response.invocation_id.as_deref(), Some("mcp-7"));
        assert!(response.ok);
        assert_eq!(response.value["result"]["ok"], true);
    }

    #[test]
    fn worka_client_is_debug_and_clone_for_generated_pack_servers() {
        let client = WorkaClient {
            socket_path: "/run/worka/broker.sock".to_string(),
        };
        let cloned = client.clone();

        assert!(format!("{client:?}").contains("WorkaClient"));
        assert_eq!(cloned.socket_path, "/run/worka/broker.sock");
    }

    #[test]
    fn worka_invocation_meta_extracts_ucan_and_invocation_id() {
        let mut meta = crate::model::Meta::new();
        meta.insert(
            "invocation_id".to_string(),
            JsonValue::String("inv-1".to_string()),
        );
        meta.insert("ucan".to_string(), JsonValue::String("token".to_string()));

        let invocation = WorkaInvocationMeta::from_meta(&meta).unwrap();

        assert_eq!(invocation.invocation_id, "inv-1");
        assert_eq!(invocation.ucan, "token");
    }

    #[test]
    fn worka_invocation_meta_rejects_missing_ucan() {
        let mut meta = crate::model::Meta::new();
        meta.insert(
            "invocation_id".to_string(),
            JsonValue::String("inv-1".to_string()),
        );

        let error = WorkaInvocationMeta::from_meta(&meta).unwrap_err();

        assert!(error.to_string().contains("missing ucan"));
    }
}
