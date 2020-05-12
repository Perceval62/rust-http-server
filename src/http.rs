
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::thread;
use std::net::SocketAddr;

enum SupportedHttpCodes
{
    //success codes
    Ok200,  //ok

    //Client errors
    Err400, //bad request
    Err404, //not found

    //Server errors
    Err501,
}

pub struct Http {
    root_path: String,
    tcp_listener: std::net::TcpListener,
    is_active: bool
}

impl Http {
    /// Returns a basic http backend
    ///
    /// # Arguments
    ///
    /// * `path` - A string representing the root path of the website's files
    /// * `host_addr` - A socket address to bind the server to.
    ///
    /// # Example
    ///
    /// ```
    ///     let http_obj = http::new("./", "127.0.0.1:80".parse().unwrap());
    /// ```
    pub fn new(path: String, host_addr: SocketAddr) -> Self
    {
        //Bind the given socket and store the listener
        let listener = std::net::TcpListener::bind(host_addr).unwrap();
        
        //Instantiate the object
        Http{ root_path: path, tcp_listener: listener, is_active: false }
    }


    /// Starts accepting requests.
    ///
    /// # Arguments
    ///
    ///
    /// # Example
    ///
    /// ```
    ///     let http_obj = http::new("./", "127.0.0.1:80".parse().unwrap());
    ///     http_obj.start();
    /// ```
    ///     
    pub fn start(&mut self)
    {
        self.is_active = true;
        println!("< http backend: starting >");
        while self.is_active
        {
            //this is not blocking, 
            let incoming_connections = self.tcp_listener.accept();
            let stream = incoming_connections.unwrap().0.try_clone();
            println!("< http backend: incoming connection >");
            //delay to not over burden other processes
            thread::sleep(std::time::Duration::from_millis(30));
            //Call static module function
            Http::handle_client(stream.unwrap(), self.root_path.clone());
            }
    
    }


    /// Creates a thread to send data over tcp/ip
    ///
    /// # Arguments
    /// * `incoming_connection` - A TcpStream that can be read, ideally has a properly formatted get request
    ///     inside it 
    /// # Example
    ///
    ///  
    fn handle_client(incoming_connection: std::net::TcpStream, mut root: String)
    {
        println!("< http backend: received a tcp connection, starting thread >");
        //incoming_connection.set_nonblocking(true).unwrap();
        std::thread::spawn(move || {

                //get the stream from the incoming tcp connection
                let mut stream = incoming_connection;
                let mut buf: [u8; 512] = [0; 512];
                //Read the incoming get request
                stream.read(&mut buf).unwrap();
                let http_request = std::str::from_utf8(&buf).unwrap();
                //parse the request to get a path
                let file_to_send = Http::parse_request(http_request, root.as_mut_str());
                match file_to_send
                {
                    Ok(file_content) => Http::send_file(file_content, stream),
                    Err(err_code) => {      
                                            let err = Http::generate_http_header(err_code);
                                            println!("{}", err);
                                            stream.write(err.as_ref()).unwrap(); 
                                            return false;
                                        },
                }
        });     
    }

    /// Stops the http server
    ///
    /// # Arguments
    /// * `self` - The object to run this function on.
    ///
    /// # Example
    ///
    /// ```
    ///     let http_obj = http::new("./", "127.0.0.1:80".parse().unwrap());
    ///     http_obj.start();
    ///     Thread::sleep_ms(1000);
    ///     http_obj.stop();
    /// ```
    ///  
    pub fn stop(mut self)
    {
        self.is_active = false;
    }

    fn parse_request(http_request: &str, root: &str) -> Result<Vec<u8>, SupportedHttpCodes>
    {
        println!("{}", http_request);
        let mut element_iter = http_request.split_whitespace();

        //If it is a get request
        if element_iter.next() == Some("GET")
        {
            let mut url: String = String::from(element_iter.next().expect("Get request invalid."));
            if url == Some("/").unwrap()
            {
                url = "index.html".to_string();
            }

            let full_url = format!("{}{}", root, url);
            println!("{}", full_url);
            
            //if path contains /app/
            //  we are dealing with a microservice
            if url.contains("/app/")
            {
                //todo
                println!("< http backend: Todo, microservices >");
                return Err(SupportedHttpCodes::Err501);
            }
            else
            {

                //else return path of html file
                let path = std::path::Path::new( &full_url );

                let mut file_content = Vec::new();
                let file = File::open(&path);

                match file {
                    Ok(mut file_obj) => {
                        file_obj.read_to_end(&mut file_content).expect("Unable to read");
                        return Ok(file_content);
                    },
                    Err(err) => return Err(SupportedHttpCodes::Err404),
                }
            }
        }
        else
        {
            return Err(SupportedHttpCodes::Err400);
        }
    }

    /// Stops the http server
    ///
    /// # Arguments
    /// * `data` - The object to run this function on.
    /// * `stream` - The object to run this function on.
    /// 
    /// # Example
    ///
    /// ```
    ///     let http_obj = http::new("./", "127.0.0.1:80".parse().unwrap());
    ///     http_obj.start();
    ///     Thread::sleep_ms(1000);
    ///     http_obj.stop();
    /// ```
    /// 
    fn send_file(data: Vec<u8>, mut stream: std::net::TcpStream) -> bool
    {
        let mut content = Http::generate_http_header(SupportedHttpCodes::Ok200).into_bytes();
        content.extend(data);

        let print_string = String::from_utf8_lossy(content.as_ref());
        println!("{}", print_string);

        let ret_status = stream.write(content.as_ref());
        stream.flush().unwrap();
        let ret: bool;
        match ret_status
        {
            Ok(ok) => ret = true,
            Err(err) => ret = false
        };
        return ret;
    }

    fn generate_http_header(http_code: SupportedHttpCodes) -> String
    {
        match http_code
        {
            SupportedHttpCodes::Ok200 => return String::from("HTTP/1.1 200 OK\nAccess-Control-Allow-Origin: *\r\n\r\n"),


            SupportedHttpCodes::Err400 => return String::from("HTTP/1.1 400 Bad Request\nAccess-Control-Allow-Origin: *\r\n\r\n"),
            SupportedHttpCodes::Err404 => return String::from("HTTP/1.1 404 Not Found\nAccess-Control-Allow-Origin: *\r\n\r\n"),

            SupportedHttpCodes::Err501 => return String::from("HTTP/1.1 501 Not Implemented\nAccess-Control-Allow-Origin: *\r\n\r\n"),
        }
    }

}