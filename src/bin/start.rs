use clap::{Parser};
use order_service::server::{server, base::Server};
// use config::{Config, ConfigError, Environment, File};

pub const CONFIG_FILE_PATH: &str = "./config/dev.toml";
pub const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
}

fn main(){
    let args = Args::parse();
    print!("{}",args.config);

    let mut server = server::Server::new(
        "order-service".to_string(),
        "localhost".to_string(),
        8080,
    );

    server.start();
}