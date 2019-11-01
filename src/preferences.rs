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
    let ret = File::open(Path::new("./pref.json"));
    /* If file doesn't exist */
    let preferences_file = match ret{
        Ok(handle) => handle,
        Err(err) => {println!("File error: {}",err);create_default_file(); File::open(Path::new("./pref.json")).unwrap()}
    };


    /* Get port and socket information from a JSON file*/
    let mut reader = std::io::BufReader::new(preferences_file);

    let mut content_string: String = String::new();

    reader.read_to_string(&mut content_string).unwrap();

    let json_object = serde_json::from_str(&content_string);

    if json_object.is_ok(){
        /* Reading a json object of type pref */
        let objects: Pref = json_object.unwrap();
        /* Parse JSON */
        let ip_string = format!("{}:{}", objects.ip, objects.port);
        println!("[Pref]: Read {} from pref.json", ip_string);
        let ret: SocketAddr = ip_string.parse().unwrap();
        Ok((ret, objects.num_threads_max))
    }
    else {
        Err("Couldn't get preferences. Check Json formatting")
    }

}

/* Will panic if file cannot be created in the same directory */
pub fn create_default_file()
{
    /* create a dummy JSON preference file */
    let json_file = json!( {"ip":"127.0.0.1","port":80, "num_threads_max":20} );
    /* Create the file */
    let file = match File::create("./pref.json")
    {
        Ok(file_handle) => file_handle,
        Err(_err) => panic!("Check files permissions, could not write preferences file.")
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
}
