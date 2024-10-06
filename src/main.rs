use clap::Parser;
use tcp::client::create_client;
use tcp::server::create_mock_server;

use std::thread;

mod tcp;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    #[arg(short, long, default_value_t = false)]
    mock_server: bool,
}

fn main() {
    let args = Args::parse();

    println!(
        "address:{} - port:{} - mock:{}",
        args.address, args.port, args.mock_server
    );

    if args.mock_server {
        thread::spawn(move || create_mock_server());
    }

    match create_client(args.address, args.port) {
        Ok(_) => (),
        Err(e) => {
            panic!("{}", e)
        }
    }
}
