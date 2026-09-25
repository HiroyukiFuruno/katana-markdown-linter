# Dependency update evidence

upstream release: `rmcp` 3.4.1 was selected by `just update` from crates.io.

api migration note: Replaced deprecated `ServerInfo` with `ServerConfig` and
mapped the removed stateless transport setting to `with_legacy_session_mode(false)`.

verification evidence: `just VERSION=v0.19.5 release-check` passed, including
the MCP stdio and remote smoke tests.
