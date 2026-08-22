use std::sync::OnceLock;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{
    Peer, RoleServer,
    model::{ClientResult, CustomRequest, ServerRequest},
    service::RequestContext,
};

/// Typed Worka broker client carried over the pack's existing MCP connection.
#[derive(Default)]
pub struct WorkaClient {
    peer: OnceLock<Peer<RoleServer>>,
}

impl std::fmt::Debug for WorkaClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorkaClient")
            .field("bound", &self.peer.get().is_some())
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkaInvocationMeta {
    pub invocation_id: String,
    pub ucan: String,
}

impl WorkaInvocationMeta {
    pub fn from_context(context: &RequestContext<RoleServer>) -> Result<Self> {
        let invocation_id = context
            .meta
            .0
            .get("invocation_id")
            .and_then(JsonValue::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| anyhow!("Worka MCP request metadata is missing invocation_id"))?;
        let ucan = context
            .meta
            .0
            .get("ucan")
            .and_then(JsonValue::as_str)
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkaDatabaseStatement {
    pub sql: String,
    #[serde(default)]
    pub parameters: Vec<JsonValue>,
}

impl WorkaClient {
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind this pack-local client to its single, process-lifetime MCP peer.
    pub fn bind(&self, context: &RequestContext<RoleServer>) {
        let _ = self.peer.set(context.peer.clone());
    }

    pub fn invocation(&self, context: &RequestContext<RoleServer>) -> Result<WorkaInvocationMeta> {
        self.bind(context);
        WorkaInvocationMeta::from_context(context)
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

    pub async fn integration_credentials(
        &self,
        invocation_id: &str,
        ucan: &str,
    ) -> Result<JsonValue> {
        self.send_request(WorkaSocketRequest {
            invocation_id: invocation_id.to_string(),
            parent_invocation_id: None,
            ucan: ucan.to_string(),
            cap: None,
            op: "integration.credentials".to_string(),
            args: JsonValue::Null,
        })
        .await
    }

    pub async fn database_execute_for_invocation(
        &self,
        invocation: &WorkaInvocationMeta,
        child_invocation_id: &str,
        group: &str,
        statement: WorkaDatabaseStatement,
    ) -> Result<JsonValue> {
        self.database_request_for_invocation(
            invocation,
            child_invocation_id,
            "worka.db.execute",
            serde_json::json!({
                "group": group,
                "sql": statement.sql,
                "parameters": statement.parameters,
            }),
        )
        .await
    }

    pub async fn database_query_for_invocation(
        &self,
        invocation: &WorkaInvocationMeta,
        child_invocation_id: &str,
        group: &str,
        statement: WorkaDatabaseStatement,
    ) -> Result<JsonValue> {
        self.database_request_for_invocation(
            invocation,
            child_invocation_id,
            "worka.db.query",
            serde_json::json!({
                "group": group,
                "sql": statement.sql,
                "parameters": statement.parameters,
            }),
        )
        .await
    }

    pub async fn database_batch_for_invocation(
        &self,
        invocation: &WorkaInvocationMeta,
        child_invocation_id: &str,
        group: &str,
        statements: Vec<WorkaDatabaseStatement>,
    ) -> Result<JsonValue> {
        self.database_request_for_invocation(
            invocation,
            child_invocation_id,
            "worka.db.batch",
            serde_json::json!({"group": group, "statements": statements}),
        )
        .await
    }

    async fn database_request_for_invocation(
        &self,
        invocation: &WorkaInvocationMeta,
        child_invocation_id: &str,
        operation: &str,
        args: JsonValue,
    ) -> Result<JsonValue> {
        self.send_request(WorkaSocketRequest {
            invocation_id: child_invocation_id.to_string(),
            parent_invocation_id: Some(invocation.invocation_id.clone()),
            ucan: invocation.ucan.clone(),
            cap: None,
            op: operation.to_string(),
            args,
        })
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
        self.send_request(WorkaSocketRequest {
            invocation_id: invocation_id.to_string(),
            parent_invocation_id: parent_invocation_id.map(str::to_string),
            ucan: ucan.to_string(),
            cap: None,
            op: "http.request".to_string(),
            args: serde_json::json!({
                "method": method,
                "url": url,
                "headers": headers.unwrap_or_default(),
                "body": body,
            }),
        })
        .await
    }

    pub async fn send_request(&self, request: WorkaSocketRequest) -> Result<JsonValue> {
        let peer = self
            .peer
            .get()
            .ok_or_else(|| anyhow!("WorkaClient is not bound to an MCP request context"))?;
        let method = request.op.clone();
        let expected_invocation_id = request.invocation_id.clone();
        let result = peer
            .send_request(ServerRequest::CustomRequest(CustomRequest::new(
                method,
                Some(serde_json::to_value(request)?),
            )))
            .await?;
        let ClientResult::CustomResult(result) = result else {
            return Err(anyhow!("Worka broker returned a non-custom MCP result"));
        };
        let response: WorkaSocketResponse = result.result_as()?;
        if response.invocation_id.as_deref() != Some(expected_invocation_id.as_str()) {
            return Err(anyhow!(
                "Worka response invocation_id mismatch: expected {}, got {:?}",
                expected_invocation_id,
                response.invocation_id
            ));
        }
        if response.ok {
            Ok(response.value)
        } else {
            Err(anyhow!(response.error.unwrap_or_else(|| {
                "Unknown Worka broker error".to_string()
            })))
        }
    }
}
