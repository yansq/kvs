use clap::Parser;
use kvs::{Command, KvStore, Result};
use std::env;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Debug, Parser)]
#[command(name = "kvs-client", author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let cur_dir = env::current_dir()?;
    let mut kv_store = KvStore::open(cur_dir)?;
    let args = Cli::parse();
    match args.command {
        Command::Set { key, value } => kv_store.set(key, value),
        Command::Get { key } => {
            if let Ok(Some(value)) = kv_store.get(key) {
                info!("{}", value);
            }
            Ok(())
        }
        Command::Rm { key } => kv_store.remove(key),
    }
}
