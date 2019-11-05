
use serde::Serialize;
use serde::Deserialize;

use std::net::{SocketAddr};

use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Microservice
{
    address: SocketAddr,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Pref {
    ip: String,
    port: u16,
    num_threads_max: u16,
    root_path: String,
    microservices: Vec<Microservice>
}

/* Returns a tcp SocketAddr type describing the user config */
pub fn get_server_settings() ->std::result::Result<(SocketAddr, u16, String), &'static str>
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
    /* Get a json serde object from the bytes that were read */
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
        Ok((ret, objects.num_threads_max, objects.root_path))
    }
    else {
        Err("\n[Pref] Error: Couldn't get preferences. Check Json formatting")
    }
}

pub fn create_default_file()
{
    /* create a dummy JSON preference file */
    let json_file = json!( {"ip":"127.0.0.1","port":80, "num_threads_max":20, "root_path":".", "microservices": []} );
    /* Create the file */
    let file = match File::create("./config.json")
    {
        Ok(file_handle) => file_handle,
        Err(_err) => panic!("   [Pref] Error: Check files permissions, \n
                                could not write config file.")
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
    println!("[Pref] Log: Created default config file.");
}
