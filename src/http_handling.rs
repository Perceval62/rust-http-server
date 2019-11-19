
use crate::config::Microservice;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::net::SocketAddr;
use threadpool::ThreadPool;

use crate::microservice;

pub fn start(address: SocketAddr, max_thread_count: u16, root_path: String, microservice_list: Vec<Microservice>)
{
    println!("[Main] Log: Got {} maximum thread.", max_thread_count);
    let num_threads_max: u16 = max_thread_count;
    println!("[Main] Log: Binding the server to {}", address);
    let ret = std::net::TcpListener::bind(address);
    let listener = match ret
    {
        Ok(listener) => {listener},
        Err(err) => {   println!("[Main] Log: could not bind address. Check for admin privileges.");
                        panic!(err)}
    };
    //let mut thread_list = Vec::with_capacity(num_threads_max as usize);
    println!("[Main] Log: Started the listener thread pool with {} maximum threads", num_threads_max);
    let pool = ThreadPool::new(num_threads_max as usize);
    loop{
        let tcp_streams = listener.accept().unwrap();
        let list = microservice_list.clone();

        let copy_root_path: String = root_path.clone();
        pool.execute(move ||
        {
            match handle_client(tcp_streams.0, copy_root_path, list)
            {
                Ok(_) => println!("[Client handling thread] Log: Client request handled"),
                Err(err) => println!("[Client handling thread] Error: Shutting down thread because of : following error\n{}", err),
            }
        });
    }
}
pub fn print_man()
{
    println!(
        "Rust Server Backend: Vincent Perrier\n
        cargo-run generate-config  -> Generates a default \"config.json\" file.\n
        cargo-run help             -> Prints program usage.\n
        cargo-run start            -> Starts the web server\n"
    );
}

fn handle_client(mut stream: std::net::TcpStream, root_path: String, microservice_list: Vec<Microservice>) -> Result<(), &'static str>
{
    println!("[Main] Log: Incoming connections from: {}", stream.peer_addr().unwrap());

    /* Get the request and split the words by whitespace */
    let mut buffer: [u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();
    let http_request = std::str::from_utf8(&buffer).unwrap();
    let split_buffer = std::str::from_utf8(&buffer).unwrap();
    let mut elements_iter = split_buffer.split_whitespace();

    println!("[Client handling thread] Log: Received http request.");

    let requested_file: &Path;      //Preparing a buffer for the Path
    let mut file_content: Vec<u8> = Vec::new();  //Preparing a buffer to store the file content

    /* Get the command of the request */
    if elements_iter.next() == Some("GET")
    {
        let path_string = elements_iter.next().expect("Get request invalid.");
        //Si la requete contient le chemin /app/
        if path_string.contains("/app/")
        {
            //rediriger la requete
            println!("[Client handling thread]: Functionnality not finished yet.");
            let ret = microservice::parse_request_string(path_string, microservice_list);
            let microservice_addr: SocketAddr;
            match ret {
                Ok(socket) => {
                    println!("[Client handling thread] Log: redirecting.");
                    microservice_addr = socket
                }
                Err(err) => {
                    println!("[Client handling thread] Log: {}", err);
                    panic!("[Client handling thread] Error: thread terminated.")
                }
            }
            let ret = microservice::redirect_request(microservice_addr, http_request.to_string());
            match ret{
                Ok(()) => println!("[Client handling thread] Log: redirection was succeful."),
                Err(()) => println!("[Client handling thread] Error: redirection failed.")
            }
        }
        else
        {
            //sinon, déservir le chemin demandé
            requested_file = Path::new(&path_string);
            let ret = get_file(requested_file, root_path);
            match ret
            {
                /* if file exists */
                Ok(ok_val) => file_content = ok_val,
                /* if file doesnt exist, create a temporary one with 404 error*/
                Err(_err)       => {
                                let response: Vec<u8> = String::from("HTTP/1.1 404 Not found\r\n\r\n<h1>404 Not found.</h1>").into_bytes();
                                stream.write(&response).unwrap();
                                println!("[Client handling thread] Warning: Requested file could not be opened");
                                ()
                            }
            };

            if file_content.is_empty()
            {
                return Err("[Client handling thread] Warning: client requested file that is not existant or outside of html root folder.");
            }

            let mut response: Vec<u8> = String::from("HTTP/1.1 200 OK\r\n\r\n").into_bytes();
            response.append(&mut file_content);
            stream.write(&response).unwrap();
        }
    }
    Ok(())
}

fn get_file(mut requested_file: &Path, root_path: String) -> Result<Vec<u8>, ()> {
    let index_html_string = "index.html";

    let compare_str = requested_file.to_str().unwrap();

    if compare_str == "/"
    {
        requested_file = Path::new(index_html_string);
    }

    let string = &format!("{}{}", root_path, requested_file.to_str().unwrap());
    println!(
        "[Client handling thread] Log: Get request is asking for {}",
        string
    );
    let copy = std::path::Path::new(string);

    let mut ret_string: Vec<u8> = Vec::new();

    let ret = File::open(copy);
    let file: std::fs::File = match ret{
        Ok(ok_file) => {
            println!("[Client handling thread] Log: fetched file from local server.");
            ok_file
        }
        Err(err) => {
            println!("[Client handling thread] Error: {}", err);
            return Err(());
        }
    };

    let mut reader = std::io::BufReader::new(file);

    reader.read_to_end(&mut ret_string).unwrap();

    Ok(ret_string)
}
