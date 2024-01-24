use clap::Parser;
use kvs::Result;

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
    let mut ip_port = "127.0.0.1:4000".to_string();
    let cli = ServerCli::parse();
    println!("{:#?}", cli);

    if let Some(addr) = cli.addr {
        ip_port = addr;
    }
    Ok(())
}
