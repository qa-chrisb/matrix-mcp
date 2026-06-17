//! Matrix MCP server.
//!
//! Exposes Matrix chat operations (login, list rooms, read/send messages, join
//! rooms) as Model Context Protocol tools over stdio, built on the official
//! `rmcp` and `matrix-sdk` Rust SDKs.

mod matrix;
mod server;

use std::sync::Arc;

use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::EnvFilter;

use crate::matrix::MatrixManager;
use crate::server::MatrixServer;

#[tokio::main]
async fn main() -> Result<()> {
    // IMPORTANT: stdout is reserved for the MCP JSON-RPC stream, so all logging
    // must go to stderr.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("matrix_mcp=info")),
        )
        .init();

    let manager = MatrixManager::from_env()?;

    match manager.try_restore().await {
        Ok(true) => tracing::info!("restored existing Matrix session"),
        Ok(false) => tracing::info!("no saved session found; use the `login` tool to authenticate"),
        Err(e) => tracing::warn!("could not restore saved session: {e:#}"),
    }

    if let Err(e) = manager.maybe_login_from_env().await {
        tracing::warn!("automatic login from environment failed: {e:#}");
    }

    let server = MatrixServer::new(Arc::new(manager));
    tracing::info!("starting Matrix MCP server on stdio");

    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
