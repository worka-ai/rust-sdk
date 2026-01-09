use std::sync::Arc;

use rmcp::{
    ClientHandler, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolRequestParam, ClientInfo, CustomNotification, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::{Mutex, Notify};

#[derive(Debug, Clone)]
struct EchoServer {
    tool_router: ToolRouter<Self>,
}

impl EchoServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct EchoRequest {
    input: String,
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for EchoServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Echo server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tool_router(router = tool_router)]
impl EchoServer {
    #[tool(description = "Echo the input string")]
    fn echo(&self, Parameters(request): Parameters<EchoRequest>) -> String {
        request.input
    }
}

#[derive(Debug, Clone, Default)]
struct DummyClient;

impl ClientHandler for DummyClient {
    fn get_info(&self) -> ClientInfo {
        ClientInfo::default()
    }
}

#[tokio::test]
async fn test_inproc_tool_call() -> anyhow::Result<()> {
    let (client_transport, server_transport) = rmcp::transport::inproc::channel();

    tokio::spawn(async move {
        let server = EchoServer::new().serve(server_transport).await?;
        server.waiting().await?;
        anyhow::Ok(())
    });

    let client = DummyClient::default().serve(client_transport).await?;

    let result = client
        .call_tool(CallToolRequestParam {
            name: "echo".into(),
            arguments: Some(json!({ "input": "hello" }).as_object().unwrap().clone()),
            task: None,
        })
        .await?;

    let text = result
        .content
        .first()
        .and_then(|content| content.raw.as_text())
        .map(|text| text.text.as_str())
        .unwrap_or_default();

    assert_eq!(text, "hello");
    client.cancel().await?;
    Ok(())
}

struct NotifyServer {
    tool_router: ToolRouter<Self>,
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for NotifyServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Notify server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }

    async fn on_initialized(&self, context: rmcp::service::NotificationContext<rmcp::RoleServer>) {
        let peer = context.peer.clone();
        tokio::spawn(async move {
            let _ = peer
                .send_notification(rmcp::model::ServerNotification::CustomNotification(
                    CustomNotification::new(
                        "notifications/inproc-ready",
                        Some(json!({ "ok": true })),
                    ),
                ))
                .await;
        });
    }
}

#[tool_router(router = tool_router)]
impl NotifyServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[derive(Default)]
struct NotifyClient {
    payload: Arc<Mutex<Option<serde_json::Value>>>,
    signal: Arc<Notify>,
}

impl ClientHandler for NotifyClient {
    fn get_info(&self) -> ClientInfo {
        ClientInfo::default()
    }

    async fn on_custom_notification(
        &self,
        notification: CustomNotification,
        _context: rmcp::service::NotificationContext<rmcp::RoleClient>,
    ) {
        let CustomNotification { params, .. } = notification;
        *self.payload.lock().await = params;
        self.signal.notify_one();
    }
}

#[tokio::test]
async fn test_inproc_notification() -> anyhow::Result<()> {
    let (client_transport, server_transport) = rmcp::transport::inproc::channel();

    tokio::spawn(async move {
        let server = NotifyServer::new().serve(server_transport).await?;
        server.waiting().await?;
        anyhow::Ok(())
    });

    let payload = Arc::new(Mutex::new(None));
    let signal = Arc::new(Notify::new());
    let client = NotifyClient {
        payload: payload.clone(),
        signal: signal.clone(),
    }
    .serve(client_transport)
    .await?;

    tokio::time::timeout(std::time::Duration::from_secs(2), signal.notified()).await?;
    assert_eq!(*payload.lock().await, Some(json!({ "ok": true })));
    client.cancel().await?;
    Ok(())
}
