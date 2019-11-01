mod preferences;
#[macro_use]
extern crate serde_json;

use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

use std::env;

fn main() {

    /* Get command line arguments */
    let args: Vec<String> = env::args().collect();
    

    println!("[Main]: Vincent Perrier Rust Server Backend.");
    let pref = preferences::get_server_info().unwrap();
    start(pref.0, pref.1);

}

fn start(address: SocketAddr, max_thread_count: u16)
{
    println!("[Main]: Got {} to bind the server.", max_thread_count);
    let num_threads_max: u16 = max_thread_count;
    let ret = std::net::TcpListener::bind(address);
    let listener = match ret
    {
        Ok(listener) => {listener},
        Err(err) => {println!("[Main]: could not bind address. Check admin privileges."); panic!(err)}
    };
    for tcp_streams in listener.incoming()
    {
        handle_client(tcp_streams.unwrap());
    }
}

fn handle_client(stream: std::net::TcpStream)
{
    println!("[Main]: Incoming connections from: {}", stream.peer_addr().unwrap());
}
