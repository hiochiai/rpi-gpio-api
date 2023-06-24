mod api;

use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use std::result::Result;

#[derive(Debug, Parser)]
#[command(
    name = "RPi GPIO API",
    author = "Hiroshi Ochiai",
    version = "0.1.0",
    about = "RPi GPIO API is a REST API server that can turn on/off GPIO of Raspberry Pi."
)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Start REST API server
    Serve {
        /// REST API server address
        #[arg(
            long,
            require_equals = true,
            num_args = 0..=1,
            default_value =  "0.0.0.0:8008",
            value_parser = socket_addr
        )]
        address: SocketAddr,
    },
}

fn socket_addr(s: &str) -> Result<SocketAddr, String> {
    match s.parse::<SocketAddr>() {
        Ok(addr) => Ok(addr),
        Err(e) => Err(format!("Invalid address: {}", e)),
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.subcommand {
        SubCommands::Serve { address } => serve(address).await,
    }
}

pub async fn serve(addr: std::net::SocketAddr) {
    println!("api server starts with {}", addr);

    let router = api::new_router();

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    if let Err(e) = tokio::signal::ctrl_c().await {
        panic!("Unable to listen for shutdown signal: {}", e);
    }
}
