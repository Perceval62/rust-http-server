use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::net::SocketAddr;
use threadpool::ThreadPool;



pub fn start(address: SocketAddr, max_thread_count: u16)
{
    println!("[Main] Log: Got {} maximum thread.", max_thread_count);
    let num_threads_max: u16 = max_thread_count;
    println!("[Main] Log: Binding the server to {}", address);
    let ret = std::net::TcpListener::bind(address);
    let listener = match ret
    {
        Ok(listener) => {listener},
        Err(err) => {println!("[Main] Log: could not bind address. Check for admin privileges.");
        panic!(err)}
    };
    //let mut thread_list = Vec::with_capacity(num_threads_max as usize);
    println!("[Main] Log: Started the listener thread pool with {} maximum threads", num_threads_max);

    let pool = ThreadPool::new(num_threads_max as usize);
    for tcp_streams in listener.incoming()
    {
        pool.execute(move ||
            {
                match handle_client(tcp_streams.unwrap())
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

fn handle_client(mut stream: std::net::TcpStream) -> Result<(), &'static str>
{
    println!("[Main] Log: Incoming connections from: {}", stream.peer_addr().unwrap());
    /* handle client */

    /* create buffer with elements of type u8, with a length of 512 elements */
    let mut buffer: [u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();
    /* Split request into its elements */
    let split_buffer = std::str::from_utf8(&buffer).unwrap();
    let mut elements_iter = split_buffer.split_whitespace();

    let _filepath: &Path;

    let mut _file_content: Vec<u8>;

    if elements_iter.next() == Some("GET")
    {
        let copy = elements_iter.next().expect("End of request"); //clone_into(&mut file_string_buf);

        let _filepath = Path::new(&copy);

        _file_content = get_file(_filepath);

        stream.write(&_file_content).unwrap();
    }
    Ok(())
}

fn get_file_extension()
{

}

fn get_file( filepath: & Path) -> Vec<u8>
{
    /* Format the filepath into an actual posix filepath (get requests start with / therefore,
       adding a . will make it ./stuff.thing) */
    let string = &format!(".{}", filepath.to_str().unwrap());

    /* From the new string build a new path object */
    let mut copy = std::path::Path::new(string);

    /* In case nothing is specified, replace the path with index.html */
    let index_html_string = "./index.html".to_string();
    if copy.to_str().unwrap() == "./"
    {
        copy = Path::new(&index_html_string);
    }

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
