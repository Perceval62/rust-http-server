#[allow(unused_imports)]
use serde_json::{Result, Value};
use serde::Serialize;
use serde::Deserialize;

#[allow(unused_imports)]
use std::net::{SocketAddr, ToSocketAddrs};
#[allow(unused_imports)]
use std::net::Ipv4Addr;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use std::io::Write;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::vec::Vec;
#[allow(unused_imports)]
use std::vec::Splice;
use std::path::Path;


/**
*   This module handles the preferences.
*   get_server_info()       Returns a SocketAddr struct and a u16 in a tuple. If config.json
*                           doesn't exist it will create one.
*                               ->SocketAddr is the socket address.
*                               ->u16 is the max number of threads
*   create_default_file()   Creates a json file in current working directory
*/

/* Serde json needs this macro to know the type of the json
object we are saving and reading.*/

/* Easy  way to remove the "" in the returned string.
Because now it is explicitly a string */
#[derive(Serialize, Deserialize)]
struct Pref {
    ip: String,
    port: u16,
    num_threads_max: u16,
}

/* Returns a tcp SocketAddr type describing the user config */
pub fn get_server_info() ->std::result::Result<(SocketAddr, u16), &'static str>
{
    /* Open file*/
    let ret = File::open(Path::new("./config.json"));
    let preferences_file = match ret{
        Ok(handle) => handle,
        Err(err) => {   println!("[Pref] Log: File error: {}.",err);
                        create_default_file();
                        File::open(Path::new("./config.json")).unwrap()
                    }
    };
    /* Get port and socket information from a JSON file*/
    let mut reader = std::io::BufReader::new(preferences_file);
    /* Buffer for the file content */
    let mut content_string: String = String::new();
    /* Get the text from file to buffer with the help of our reader object */
    reader.read_to_string(&mut content_string).unwrap();
    /* Get a json serde object */
    let json_object = serde_json::from_str(&content_string);
    /* If JSON formatting is ok*/
    if json_object.is_ok(){
        /* Read json object of type pref */
        let objects: Pref = json_object.unwrap();
        /* Parse JSON into a string */
        let ip_string = format!("{}:{}", objects.ip, objects.port);
        println!("[Pref] Log: Read {} from config.json", ip_string);
        /* Create a SocketAddr object from the string */
        let ret: SocketAddr = ip_string.parse().unwrap();
        /* return the SocketAddr object and the max num thread in the preferences files */
        Ok((ret, objects.num_threads_max))
    }
    else {
        Err("\n[Pref] Error: Couldn't get preferences. Check Json formatting")
    }
}

pub fn create_default_file()
{
    /* create a dummy JSON preference file */
    let json_file = json!( {"ip":"127.0.0.1","port":80, "num_threads_max":20} );
    /* Create the file */
    let file = match File::create("./config.json")
    {
        Ok(file_handle) => file_handle,
        Err(_err) => panic!("[Pref] Error: Check files permissions, could not write preferences file.")
    };
    /* Convert the serde strructure to a rust string  */
    let data = serde_json::to_string_pretty(&json_file).unwrap();
    /* prepare the writing buffer */
    let mut writer = std::io::BufWriter::new(file);
    /* write the rust string with json in it to file system*/
    writer.write(data.as_bytes()).unwrap();
    /* writer & file going out of scope, the file is going to be closed and dropped */
    writer.flush().unwrap();
    /*  Make sure to drop the buffer writer */
    std::mem::drop(writer);
    println!("[Pref] Log: Created default config file.");
}
