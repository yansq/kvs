use clap::Parser;
use kvs::Command;
use kvs::{KvStore, Result};
use std::env;

#[derive(Debug, Parser)]
#[command(name = "kvs-client", author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let cur_dir = env::current_dir()?;
    let mut kv_store = KvStore::open(cur_dir)?;
    let args = Cli::parse();
    match args.command {
        Command::Set { key, value } => kv_store.set(key, value),
        Command::Get { key } => {
            if let Ok(Some(value)) = kv_store.get(key) {
                println!("{}", value);
            }
            Ok(())
        }
        Command::Rm { key } => kv_store.remove(key),
    }
}
