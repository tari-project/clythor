//  Copyright 2024. The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::{fmt::Debug, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(
        short,
        long,
        aliases = &["base_path", "base_dir", "base-dir"],
        env = "TARI_BASE_DIR"
    )]
    pub base_path: Option<String>,
    #[clap(long, aliases = &["log_path", "log-dir"])]
    pub log_path: Option<PathBuf>,
    #[clap(long)]
    pub monero_base_node_address: Option<String>,
    #[clap(long, alias = "user")]
    pub monero_wallet_address: Option<String>,
    #[clap(long)]
    pub mine_until_height: Option<u64>,
    #[clap(long)]
    pub miner_max_blocks: Option<u64>,
    #[clap(long)]
    pub miner_min_diff: Option<u64>,
    #[clap(long)]
    pub miner_max_diff: Option<u64>,
    #[clap(short, long, alias = "non-interactive", env = "TARI_NON_INTERACTIVE")]
    pub non_interactive_mode: bool,
    #[clap(short = 't', long, alias = "threads")]
    pub num_mining_threads: Option<usize>,
    /// The port for the http server. Default: 18000
    #[clap(short = 'p', long, alias = "http-port")]
    pub http_port: Option<u16>,
    #[clap(short = 'r', long, alias = "refresh-interval")]
    pub template_refresh_interval_ms: Option<u64>,
}
