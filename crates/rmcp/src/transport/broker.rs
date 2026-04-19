use std::{future::Future, sync::Arc};

use anyhow::{Result, anyhow};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::UnixStream,
    sync::{Mutex, mpsc},
};
use tokio_util::{
    bytes::{BufMut, BytesMut},
    codec::{Decoder, Encoder, FramedRead, FramedWrite},
};

use crate::{
    model::JsonRpcMessage,
    service::ServiceRole,
    transport::{DynamicTransportError, Transport},
};

pub struct BrokerTransport<R: ServiceRole> {
    reader: FramedRead<
        tokio::net::unix::OwnedReadHalf,
        BrokerCodec<JsonRpcMessage<R::PeerReq, R::PeerResp, R::PeerNot>>,
    >,
    tx: mpsc::Sender<JsonRpcMessage<R::Req, R::Resp, R::Not>>,
}

impl<R: ServiceRole> BrokerTransport<R> {
    pub async fn connect(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let stream = UnixStream::connect(path).await?;
        let (read_half, write_half) = stream.into_split();

        let (tx, mut rx) = mpsc::channel::<JsonRpcMessage<R::Req, R::Resp, R::Not>>(64);

        tokio::spawn(async move {
            let mut writer = FramedWrite::new(write_half, BrokerCodec::default());
            while let Some(msg) = rx.recv().await {
                if let Err(e) = writer.send(msg).await {
                    tracing::error!("BrokerTransport writer error: {}", e);
                    break;
                }
            }
            let _ = writer.get_mut().shutdown().await;
        });

        Ok(Self {
            reader: FramedRead::new(read_half, BrokerCodec::default()),
            tx,
        })
    }
}

impl<R: ServiceRole> Transport<R> for BrokerTransport<R> {
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
        let mut reader = &mut self.reader;
        async move { reader.next().await.and_then(|r| r.ok()) }
    }

    fn close(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        let _ = self.tx.clone();
        async move { Ok(()) }
    }
}

pub struct BrokerCodec<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Default for BrokerCodec<T> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: for<'de> Deserialize<'de>> Decoder for BrokerCodec<T> {
    type Item = T;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(pos) = src.iter().position(|&b| b == b'\n') {
            let line = src.split_to(pos + 1);
            let item = serde_json::from_slice(&line[..pos])
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}

impl<T: Serialize> Encoder<T> for BrokerCodec<T> {
    type Error = std::io::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let json = serde_json::to_vec(&item)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
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

impl BrokerClient {
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
        let headers = headers.unwrap_or_default();
        let req = BrokerSocketRequest {
            invocation_id: invocation_id.to_string(),
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

        if self.socket_path.contains(':') {
            // TCP
            let stream = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                tokio::net::TcpStream::connect(&self.socket_path),
            )
            .await
            .map_err(|_| anyhow!("Broker connect timeout (TCP)"))??;
            let mut stream = stream;
            stream.write_all(&serde_json::to_vec(&req)?).await?;
            stream.write_all(b"\n").await?;

            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            reader.read_line(&mut line).await?;

            let res: BrokerSocketResponse = serde_json::from_str(&line)?;
            if res.ok {
                Ok(res.value)
            } else {
                Err(anyhow!(
                    res.error
                        .unwrap_or_else(|| "Unknown broker error".to_string())
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

            let res: BrokerSocketResponse = serde_json::from_str(&line)?;
            if res.ok {
                Ok(res.value)
            } else {
                Err(anyhow!(
                    res.error
                        .unwrap_or_else(|| "Unknown broker error".to_string())
                ))
            }
        }
    }
}
