#![allow(dead_code, unused_variables)]

use anyhow::Result as Anyhow;

pub(crate) mod completions;

mod cli;
mod server;

#[tokio::main]
async fn main() -> Anyhow<()> {
    server::run_server().await
}
