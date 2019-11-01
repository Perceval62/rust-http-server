mod preferences;

use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");
    preferences::create_create_default_file();
    println!("{}", preferences::get_server_info().unwrap());
}
