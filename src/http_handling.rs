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
    /* handle client */

    /* create buffer with elements of type u8, with a length of 512 elements */
    let mut buffer: [u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();
    /* Split request into its elements */
    let split_buffer = std::str::from_utf8(&buffer).unwrap();
    /* Split the buffer by whitespaces */
    let mut elements_iter = split_buffer.split_whitespace();
    let _filepath: &Path;
    let mut _file_content: Vec<u8>;
    if elements_iter.next() == Some("GET")
    {
        let copy = elements_iter.next().expect("End of request"); //clone_into(&mut file_string_buf);

        let _filepath = Path::new(&copy);

        _file_content = get_file(_filepath, root_path);

        let mut response: Vec<u8> = String::from("HTTP/1.1 200 OK\r\n\r\n").into_bytes();
        response.append(&mut _file_content);
        stream.write(&response).unwrap();
    }
    Ok(())
}

fn get_file( mut requested_file: &Path, root_path: String) -> Vec<u8>
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

    /* Create buffer (using u8 because String expects an encoding)*/
    /* This reads ascii */
    let mut ret_string: Vec<u8> = Vec::new();

    /* Get file handle  */
    let ret = File::open(copy);
    let file: std::fs::File = match ret
    {
        Ok(ok_file) => {println!("[Client handling thread] Log: fetched file from local server."); ok_file},
        Err(err) => panic!("[Client handling thread] Error: {}", err),
    };

    /* Get the content from the file */
    let mut reader = std::io::BufReader::new(file);

    /* read the whole file, put it in a buffer, then return it */
    reader.read_to_end(&mut ret_string).unwrap(); //(&mut ret_string).unwrap();

    return ret_string;
}
