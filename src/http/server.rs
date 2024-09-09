//   Copyright 2024. The Tari Project
//
//   Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//   following conditions are met:
//
//   1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//   disclaimer.
//
//   2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//   following disclaimer in the documentation and/or other materials provided with the distribution.
//
//   3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//   products derived from this software without specific prior written permission.
//
//   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//   INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//   DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//   SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//   WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//   USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::http::config;
use crate::http::handlers::{health, stats, version};
use crate::stats_store::StatsStore;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tari_shutdown::ShutdownSignal;
use thiserror::Error;
use tokio::io;

/// An HTTP server that provides stats and other useful information.
pub struct HttpServer {
    shutdown_signal: ShutdownSignal,
    config: config::Config,
    stats_store: Arc<StatsStore>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
}

#[derive(Clone)]
pub struct AppState {
    pub stats_store: Arc<StatsStore>,
}

impl HttpServer {
    pub fn new(shutdown_signal: ShutdownSignal, config: config::Config, stats_store: Arc<StatsStore>) -> Self {
        Self {
            shutdown_signal,
            config,
            stats_store,
        }
    }

    pub fn routes(&self) -> Router {
        Router::new()
            .route("/health", get(health::handle_health))
            .route("/version", get(version::handle_version))
            .route("/stats", get(stats::handle_get_stats))
            .with_state(AppState {
                stats_store: self.stats_store.clone(),
            })
    }

    /// Starts the http server on the port passed in ['HttpServer::new']
    pub async fn start(&self) -> Result<(), Error> {
        let router = self.routes();
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.config.port))
            .await
            .map_err(Error::IO)?;
        println!("Starting HTTP server at http://127.0.0.1:{}", self.config.port);
        axum::serve(listener, router)
            .with_graceful_shutdown(self.shutdown_signal.clone())
            .await
            .map_err(Error::IO)?;
        println!("HTTP server stopped!");
        Ok(())
    }
}
