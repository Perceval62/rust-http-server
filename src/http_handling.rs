use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::net::SocketAddr;
use threadpool::ThreadPool;

pub fn start(address: SocketAddr, max_thread_count: u16, root_path: String)
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
    for tcp_streams in listener.incoming()
    {
        let copy_root_path: String = root_path.clone();
        pool.execute(move ||
        {
            match handle_client(tcp_streams.unwrap(), copy_root_path)
            {
                Ok(_) => println!("[Client handling thread] Log: Client request handled"),
                Err(err) => println!("[Client handling thread] Error: {}", err),
            }
        }
        );
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

fn handle_client(mut stream: std::net::TcpStream, root_path: String) -> Result<(), &'static str>
{
    println!("[Main] Log: Incoming connections from: {}", stream.peer_addr().unwrap());

    /* Get the request and split the words by whitespace */
    let mut buffer: [u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();
    let split_buffer = std::str::from_utf8(&buffer).unwrap();
    let mut elements_iter = split_buffer.split_whitespace();

    println!("[Client handling thread] Log: Received http request.");

    let requested_file: &Path;      //Preparing a buffer for the Path
    let mut file_content: Vec<u8>;  //Preparing a buffer to store the file content

    /* Get the command of the request */
    if elements_iter.next() == Some("GET")
    {
        let path_string = elements_iter.next().expect("Get request invalid");
        //Si la requete contient le chemin /app/
        if path_string.contains("/app/")
        {
            //rediriger la requete
        }
        else
        {
        //sinon, déservir le chemin demandé
            requested_file = Path::new(&path_string);

            file_content = match get_file(requested_file, root_path)
            {
                /* if file exists */
                Ok(ok_val)  => ok_val,
                /* if file doesnt exist, create a temporary one with 404 error*/
                Err(_err)       => {
                                let response: Vec<u8> = String::from("HTTP/1.1 404 Not found\r\n\r\n<h1>404 Not found.</h1>").into_bytes();
                                stream.write(&response).unwrap();
                                panic!("Requested file could not be opened");
                            }
            };
            let mut response: Vec<u8> = String::from("HTTP/1.1 200 OK\r\n\r\n").into_bytes();
            response.append(&mut file_content);
            stream.write(&response).unwrap();
        }
    }
    Ok(())
}

fn get_file( mut requested_file: &Path, root_path: String) -> Result<Vec<u8>, ()>
{
    let index_html_string = "index.html";

    let compare_str = requested_file.to_str().unwrap();

    if compare_str == "/"
    {
        requested_file = Path::new(index_html_string);
    }

    let string = &format!("{}{}", root_path, requested_file.to_str().unwrap());
    println!("{}", string);
    let copy = std::path::Path::new(string);

    let mut ret_string: Vec<u8> = Vec::new();

    let ret = File::open(copy);
    let file: std::fs::File = match ret
    {
        Ok(ok_file) => {println!("[Client handling thread] Log: fetched file from local server."); ok_file},
        Err(err) => {println!("[Client handling thread] Error: {}", err); return Err(())},
    };

    let mut reader = std::io::BufReader::new(file);

    reader.read_to_end(&mut ret_string).unwrap();

    Ok(ret_string)
}
