use std::net::IpAddr;

use clap::{Parser, Subcommand};
use env_logger::Env;

mod build;
mod packet;
mod server;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Builds the balatro.vs mod
    #[command()]
    Build,

    /// Builds the balatro.vs mod and runs the game via steam
    #[command()]
    Run,

    /// Runs the balatro.vs server
    #[command()]
    Server {
        /// The address for the server
        #[arg(long)]
        addr: Option<IpAddr>,

        /// The port for the server
        #[arg(long)]
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() {
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let args = Args::parse();
    match args.subcommand {
        Subcommands::Build => build::build().await.expect("Failed to build mod"),
        Subcommands::Run => build::run().await.expect("Failed to run mod"),
        Subcommands::Server { .. } => server::server(args.into())
            .await
            .expect("Server failed to run"),
    }
}
