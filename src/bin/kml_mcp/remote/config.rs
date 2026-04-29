use std::{env, net::SocketAddr, time::Duration};

const DEFAULT_ADDR: &str = "127.0.0.1:3000";
const DEFAULT_ENDPOINT: &str = "/mcp";
const DEFAULT_MAX_BODY_BYTES: usize = 1_048_576;
const DEFAULT_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_MAX_CONCURRENCY: usize = 16;

#[derive(Clone)]
pub(super) struct RemoteConfig {
    pub(super) addr: SocketAddr,
    pub(super) endpoint: String,
    pub(super) auth: AuthConfig,
    pub(super) limits: RemoteLimits,
    pub(super) allowed_hosts: Vec<String>,
}

#[derive(Clone)]
pub(super) enum AuthConfig {
    Bearer(String),
    AnonymousTextOnly,
}

#[derive(Clone)]
pub(super) struct RemoteLimits {
    pub(super) max_body_bytes: usize,
    pub(super) request_timeout: Duration,
    pub(super) max_concurrency: usize,
}

impl RemoteConfig {
    pub(super) fn from_env() -> Result<Self, String> {
        Ok(Self {
            addr: parse_addr()?,
            endpoint: parse_endpoint()?,
            auth: parse_auth()?,
            limits: RemoteLimits {
                max_body_bytes: parse_usize(
                    "KML_MCP_REMOTE_MAX_BODY_BYTES",
                    DEFAULT_MAX_BODY_BYTES,
                )?,
                request_timeout: Duration::from_millis(parse_u64(
                    "KML_MCP_REMOTE_TIMEOUT_MS",
                    DEFAULT_TIMEOUT_MS,
                )?),
                max_concurrency: parse_usize(
                    "KML_MCP_REMOTE_MAX_CONCURRENCY",
                    DEFAULT_MAX_CONCURRENCY,
                )?,
            },
            allowed_hosts: parse_allowed_hosts()?,
        })
    }
}

fn parse_addr() -> Result<SocketAddr, String> {
    let raw = env::var("KML_MCP_REMOTE_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string());
    raw.parse()
        .map_err(|err| format!("invalid KML_MCP_REMOTE_ADDR `{raw}`: {err}"))
}

fn parse_endpoint() -> Result<String, String> {
    let endpoint =
        env::var("KML_MCP_REMOTE_ENDPOINT").unwrap_or_else(|_| DEFAULT_ENDPOINT.to_string());
    if endpoint.starts_with('/') {
        Ok(endpoint)
    } else {
        Err("KML_MCP_REMOTE_ENDPOINT must start with `/`".to_string())
    }
}

fn parse_auth() -> Result<AuthConfig, String> {
    match env::var("KML_MCP_REMOTE_TOKEN") {
        Ok(token) if !token.is_empty() => Ok(AuthConfig::Bearer(token)),
        Ok(_) => Err("KML_MCP_REMOTE_TOKEN must not be empty".to_string()),
        Err(_) if parse_bool("KML_MCP_REMOTE_ALLOW_ANONYMOUS_READ", false)? => {
            Ok(AuthConfig::AnonymousTextOnly)
        }
        Err(_) => Err(
            "KML_MCP_REMOTE_TOKEN is required unless KML_MCP_REMOTE_ALLOW_ANONYMOUS_READ=true"
                .to_string(),
        ),
    }
}

fn parse_allowed_hosts() -> Result<Vec<String>, String> {
    match env::var("KML_MCP_REMOTE_ALLOWED_HOSTS") {
        Ok(raw) => {
            let hosts = raw
                .split(',')
                .map(str::trim)
                .filter(|host| !host.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>();
            if hosts.is_empty() {
                Err("KML_MCP_REMOTE_ALLOWED_HOSTS must contain at least one host".to_string())
            } else {
                Ok(hosts)
            }
        }
        Err(_) => Ok(vec![
            "localhost".to_string(),
            "127.0.0.1".to_string(),
            "::1".to_string(),
        ]),
    }
}

fn parse_bool(name: &str, default: bool) -> Result<bool, String> {
    match env::var(name) {
        Ok(raw) => match raw.as_str() {
            "1" | "true" | "TRUE" | "yes" | "YES" => Ok(true),
            "0" | "false" | "FALSE" | "no" | "NO" => Ok(false),
            _ => Err(format!("{name} must be true or false")),
        },
        Err(_) => Ok(default),
    }
}

fn parse_usize(name: &str, default: usize) -> Result<usize, String> {
    match env::var(name) {
        Ok(raw) => raw
            .parse::<usize>()
            .map_err(|err| format!("invalid {name} `{raw}`: {err}"))
            .and_then(|value| positive_usize(name, value)),
        Err(_) => Ok(default),
    }
}

fn parse_u64(name: &str, default: u64) -> Result<u64, String> {
    match env::var(name) {
        Ok(raw) => raw
            .parse::<u64>()
            .map_err(|err| format!("invalid {name} `{raw}`: {err}"))
            .and_then(|value| positive_u64(name, value)),
        Err(_) => Ok(default),
    }
}

fn positive_usize(name: &str, value: usize) -> Result<usize, String> {
    if value == 0 {
        Err(format!("{name} must be greater than zero"))
    } else {
        Ok(value)
    }
}

fn positive_u64(name: &str, value: u64) -> Result<u64, String> {
    if value == 0 {
        Err(format!("{name} must be greater than zero"))
    } else {
        Ok(value)
    }
}
