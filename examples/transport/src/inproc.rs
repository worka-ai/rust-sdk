use rmcp::{
    ClientHandler, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolRequestParam, ClientInfo, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;
use serde_json::json;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    println!("Result: {:?}", result.content);
    client.cancel().await?;
    Ok(())
}
