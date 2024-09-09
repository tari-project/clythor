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

use std::{io::stdout, path::PathBuf};

use clap::Parser;
use crossterm::{execute, terminal::SetTitle};
use log::*;
use minotari_app_utilities::consts;
use tari_common::{exit_codes::ExitError, initialize_logging};

pub const LOG_TARGET: &str = "clythor::main";
pub const LOG_TARGET_FILE: &str = "minotari::logging::clythor::main";

mod cli;
use cli::Cli;
mod run_miner;
use run_miner::start_miner;
mod error;
use tari_common::exit_codes::ExitCode;
mod json_rpc;
use json_rpc::Request;

mod http;
mod shared_dataset;
mod stats_store;

pub fn base_path(cli: &Cli) -> PathBuf {
    let path = cli.base_path.clone().unwrap_or(env!("CARGO_MANIFEST_DIR").to_string());
    PathBuf::from(path)
}

#[tokio::main]
async fn main() {
    let terminal_title = format!("RandomX- Version {}", consts::APP_VERSION);
    if let Err(e) = execute!(stdout(), SetTitle(terminal_title.as_str())) {
        println!("Error setting terminal title. {}", e)
    }
    match main_inner().await {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            error!(target: LOG_TARGET, "Fatal error: {:?}", err);
            let exit_code = err.exit_code;
            error!(target: LOG_TARGET, "Exiting with code: {:?}", exit_code);
            std::process::exit(exit_code as i32)
        },
    }
}

async fn main_inner() -> Result<(), ExitError> {
    let cli = Cli::parse();
    initialize_logging(
        &base_path(&cli).join("config").join("clythor").join("log4rs.yml"),
        &base_path(&cli),
        include_str!("../log4rs_sample.yml"),
    )?;
    start_miner(cli)
        .await
        .map_err(|e| ExitError::new(ExitCode::UnknownError, e.to_string()))
}
