use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Debug, Parser)]
#[command(name = "kvs", author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Set {
        #[arg(value_name = "KEY")]
        key: String,
        #[arg(value_name = "VALUE")]
        value: String,
    },
    Get {
        #[arg(value_name = "KEY")]
        key: String,
    },
    Rm {
        #[arg(value_name = "KEY")]
        key: String,
    },
}

fn main() {
    let mut _kv_store: KvStore = Default::default();
    let args = Cli::parse();
    match args.command {
        Command::Set { key, value } => {
            unimplemented!("unimplemented");
            //kv_store.set(key, value);
        }
        Command::Get { key } => {
            unimplemented!("unimplemented");
            // let _value = kv_store.get(key);
        }
        Command::Rm { key } => {
            unimplemented!("unimplemented");
            // kv_store.remove(key);
        }
    }
}
