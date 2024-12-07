mod utils;
mod infra;
mod app;

use app::args::{parse_args, Command};
use app::handlers::{handle_public_ip, handle_private_ip, handle_both_ips};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    match parse_args(args){
        Command::Public => handle_public_ip().await,
        Command::Private => handle_private_ip().await,
        Command::Invalid => handle_both_ips().await,
    }
}
