use clap::Parser;
use kvs::Result;
use tracing::{info, Level};
use tracing_subscriber;

/// kvs-server [--addr IP-PORT] [--engine ENGINE-NAME]
#[derive(Debug, Parser)]
#[command(name = "kvs-server", author, version, about)]
struct ServerCli {
    #[arg(long, value_name = "IP-PORT")]
    addr: Option<String>,
    #[arg(long, value_name = "ENGINE-NAME")]
    engine: Option<String>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let mut ip_port = "127.0.0.1:4000".to_string();
    let cli = ServerCli::parse();
    info!("{:#?}", cli);

    if let Some(addr) = cli.addr {
        ip_port = addr;
    }
    Ok(())
}
