#[path = "remote/config.rs"]
mod config;
#[path = "remote/runtime.rs"]
mod runtime;
#[path = "remote/server.rs"]
mod server;
#[path = "remote/transport.rs"]
mod transport;

pub(crate) use runtime::KmlMcpRemoteRuntime;
