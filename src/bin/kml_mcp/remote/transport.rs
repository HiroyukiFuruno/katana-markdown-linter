use super::{
    config::{AuthConfig, RemoteLimits},
    server::KmlMcpRemoteServer,
};
use axum::{
    body::{to_bytes, Body},
    extract::State,
    http::{header, Request, Response, StatusCode},
};
use rmcp::transport::{
    streamable_http_server::session::never::NeverSessionManager, StreamableHttpServerConfig,
    StreamableHttpService,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::Semaphore;

const SSE_KEEP_ALIVE_SECONDS: u64 = 15;

type RemoteMcpService = StreamableHttpService<KmlMcpRemoteServer, NeverSessionManager>;

#[derive(Clone)]
pub(super) struct RemoteHttpState {
    auth: AuthConfig,
    limits: RemoteLimits,
    semaphore: Arc<Semaphore>,
    service: RemoteMcpService,
}

impl RemoteHttpState {
    pub(super) fn new(auth: AuthConfig, limits: RemoteLimits, allowed_hosts: Vec<String>) -> Self {
        let service = StreamableHttpService::new(
            || Ok(KmlMcpRemoteServer::new()),
            Arc::new(NeverSessionManager::default()),
            StreamableHttpServerConfig::default()
                .with_allowed_hosts(allowed_hosts)
                .with_stateful_mode(false)
                .with_json_response(true)
                .with_sse_keep_alive(Some(Duration::from_secs(SSE_KEEP_ALIVE_SECONDS))),
        );
        Self {
            auth,
            semaphore: Arc::new(Semaphore::new(limits.max_concurrency)),
            limits,
            service,
        }
    }
}

pub(super) async fn handle_mcp(
    State(state): State<RemoteHttpState>,
    request: Request<Body>,
) -> Response<Body> {
    if !authorized(&state.auth, &request) {
        return plain_response(StatusCode::UNAUTHORIZED, "missing or invalid bearer token");
    }
    let Ok(_permit) = state.semaphore.clone().try_acquire_owned() else {
        return plain_response(
            StatusCode::TOO_MANY_REQUESTS,
            "too many concurrent requests",
        );
    };
    let request = match read_limited_body(request, state.limits.max_body_bytes).await {
        Ok(request) => request,
        Err(response) => return response,
    };
    match tokio::time::timeout(state.limits.request_timeout, state.service.handle(request)).await {
        Ok(response) => {
            let (parts, body) = response.into_parts();
            Response::from_parts(parts, Body::new(body))
        }
        Err(_) => plain_response(StatusCode::REQUEST_TIMEOUT, "request timed out"),
    }
}

fn authorized(auth: &AuthConfig, request: &Request<Body>) -> bool {
    match auth {
        AuthConfig::AnonymousTextOnly => true,
        AuthConfig::Bearer(token) => {
            let expected = format!("Bearer {token}");
            let actual = request
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok());
            actual == Some(expected.as_str())
        }
    }
}

async fn read_limited_body(
    request: Request<Body>,
    max_body_bytes: usize,
) -> Result<Request<Body>, Response<Body>> {
    let (parts, body) = request.into_parts();
    match to_bytes(body, max_body_bytes).await {
        Ok(bytes) => Ok(Request::from_parts(parts, Body::from(bytes))),
        Err(_) => Err(plain_response(
            StatusCode::PAYLOAD_TOO_LARGE,
            "request body exceeds KML_MCP_REMOTE_MAX_BODY_BYTES",
        )),
    }
}

fn plain_response(status: StatusCode, message: &str) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from(message.to_string()))
        .expect("valid response")
}
