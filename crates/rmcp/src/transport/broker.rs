use std::future::Future;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use tokio::net::UnixStream;
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};
use tokio_util::codec::{FramedRead, FramedWrite, Decoder, Encoder};
use tokio_util::bytes::{BytesMut, BufMut, Buf};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::{SinkExt, StreamExt};

use crate::model::{JsonRpcMessage};
use crate::service::ServiceRole;
use crate::transport::{Transport, DynamicTransportError};

pub struct BrokerTransport<R: ServiceRole> {
    reader: FramedRead<tokio::net::unix::OwnedReadHalf, BrokerCodec<JsonRpcMessage<R::PeerReq, R::PeerResp, R::PeerNot>>>,
    writer: Arc<Mutex<Option<FramedWrite<tokio::net::unix::OwnedWriteHalf, BrokerCodec<JsonRpcMessage<R::Req, R::Resp, R::Not>>>>>>,
}

impl<R: ServiceRole> BrokerTransport<R> {
    pub async fn connect(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let stream = UnixStream::connect(path).await?;
        let (read_half, write_half) = stream.into_split();
        Ok(Self {
            reader: FramedRead::new(read_half, BrokerCodec::default()),
            writer: Arc::new(Mutex::new(Some(FramedWrite::new(write_half, BrokerCodec::default())))),
        })
    }
}

#[async_trait]
impl<R: ServiceRole> Transport<R> for BrokerTransport<R> {
    type Error = anyhow::Error;

    fn send(&mut self, message: JsonRpcMessage<R::Req, R::Resp, R::Not>) -> impl Future<Output = Result<(), Self::Error>> + Send + 'static {
        let writer_lock = self.writer.clone();
        async move {
            let mut writer = writer_lock.lock().await;
            if let Some(ref mut w) = *writer {
                w.send(message).await.map_err(|e| anyhow!(e))
            } else {
                Err(anyhow!("Transport closed"))
            }
        }
    }

    fn receive(&mut self) -> impl Future<Output = Option<JsonRpcMessage<R::PeerReq, R::PeerResp, R::PeerNot>>> + Send {
        let mut reader = &mut self.reader;
        async move {
            reader.next().await.and_then(|r| r.ok())
        }
    }

    fn close(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        let writer_lock = self.writer.clone();
        async move {
            let mut writer = writer_lock.lock().await;
            if let Some(mut w) = writer.take() {
                w.get_mut().shutdown().await.map_err(|e| anyhow!(e))
            } else {
                Ok(())
            }
        }
    }
}

pub struct BrokerCodec<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for BrokerCodec<T> {
    fn default() -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

impl<T: for<'de> Deserialize<'de>> Decoder for BrokerCodec<T> {
    type Item = T;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(pos) = src.iter().position(|&b| b == b'\n') {
            let line = src.split_to(pos + 1);
            let item = serde_json::from_slice(&line[..pos])?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}

impl<T: Serialize> Encoder<T> for BrokerCodec<T> {
    type Error = anyhow::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let json = serde_json::to_vec(&item)?;
        dst.extend_from_slice(&json);
        dst.put_u8(b'\n');
        Ok(())
    }
}

pub struct BrokerClient {
    socket_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct BrokerSocketRequest {
    pub invocation_id: String,
    pub ucan: String,
    pub cap: Option<String>,
    pub op: String,
    pub args: JsonValue,
}

#[derive(Serialize, Deserialize)]
pub struct BrokerSocketResponse {
    pub ok: bool,
    pub value: JsonValue,
    pub error: Option<String>,
}

impl BrokerClient {
    pub fn new() -> Self {
        let path = std::env::var("WORKA_BROKER_SOCKET").unwrap_or_else(|_| "/run/worka/broker.sock".to_string());
        Self { socket_path: path }
    }

    pub async fn http_request(&self, invocation_id: &str, ucan: &str, method: &str, url: &str, body: Option<JsonValue>) -> Result<JsonValue> {
        let req = BrokerSocketRequest {
            invocation_id: invocation_id.to_string(),
            ucan: ucan.to_string(),
            cap: None,
            op: "http.request".to_string(),
            args: serde_json::json!({
                "method": method,
                "url": url,
                "body": body,
            }),
        };

        let mut stream = UnixStream::connect(&self.socket_path).await?;
        let mut buf = serde_json::to_vec(&req)?;
        buf.push(b'\n');
        stream.write_all(&buf).await?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        
        let res: BrokerSocketResponse = serde_json::from_str(&line)?;
        if res.ok {
            Ok(res.value)
        } else {
            Err(anyhow!(res.error.unwrap_or_else(|| "Unknown broker error".to_string())))
        }
    }
}
