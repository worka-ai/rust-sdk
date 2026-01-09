use std::future::Future;

use tokio::sync::mpsc;

use super::Transport;
use crate::service::{RxJsonRpcMessage, ServiceRole, TxJsonRpcMessage};

pub const DEFAULT_INPROC_CHANNEL_CAPACITY: usize = 16;

#[derive(Debug, thiserror::Error)]
pub enum InprocTransportError {
    #[error("inproc transport closed")]
    Closed,
}

pub struct InprocTransport<R: ServiceRole> {
    tx: Option<mpsc::Sender<TxJsonRpcMessage<R>>>,
    rx: mpsc::Receiver<RxJsonRpcMessage<R>>,
}

impl<R: ServiceRole> InprocTransport<R> {
    pub fn new(
        tx: mpsc::Sender<TxJsonRpcMessage<R>>,
        rx: mpsc::Receiver<RxJsonRpcMessage<R>>,
    ) -> Self {
        Self {
            tx: Some(tx),
            rx,
        }
    }
}

impl<R: ServiceRole> Transport<R> for InprocTransport<R> {
    type Error = InprocTransportError;

    fn send(
        &mut self,
        item: TxJsonRpcMessage<R>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send + 'static {
        let tx = self.tx.clone();
        async move {
            let Some(tx) = tx else {
                return Err(InprocTransportError::Closed);
            };
            tx.send(item)
                .await
                .map_err(|_| InprocTransportError::Closed)
        }
    }

    fn receive(&mut self) -> impl Future<Output = Option<RxJsonRpcMessage<R>>> + Send {
        self.rx.recv()
    }

    fn close(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        self.tx.take();
        async { Ok(()) }
    }
}

#[cfg(all(feature = "client", feature = "server"))]
use crate::service::{RoleClient, RoleServer};

#[cfg(all(feature = "client", feature = "server"))]
pub fn channel() -> (InprocTransport<RoleClient>, InprocTransport<RoleServer>) {
    channel_with_capacity(DEFAULT_INPROC_CHANNEL_CAPACITY)
}

#[cfg(all(feature = "client", feature = "server"))]
pub fn channel_with_capacity(
    capacity: usize,
) -> (InprocTransport<RoleClient>, InprocTransport<RoleServer>) {
    let (client_tx, server_rx) = mpsc::channel::<TxJsonRpcMessage<RoleClient>>(capacity);
    let (server_tx, client_rx) = mpsc::channel::<TxJsonRpcMessage<RoleServer>>(capacity);

    let client = InprocTransport::new(client_tx, client_rx);
    let server = InprocTransport::new(server_tx, server_rx);
    (client, server)
}
