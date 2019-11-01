#[allow(unused_imports)]
use serde_json::{Result, Value};

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


/* Returns a tcp SocketAddr type describing the user config */
pub fn get_server_info() -> Result<String>
{
    /* Open file*/
print!("1\n");

    let ret = File::open("./pref.json");
    let preferences_file =
    match ret {
        Ok(file_handle) => file_handle,
        Err(_err) => {  println!("{}", _err);
                        create_create_default_file();
                        File::open("./pref.json").unwrap()},
    };
print!("1\n");
        /* Get port and socket information from a JSON file*/
        let mut reader = std::io::BufReader::new(preferences_file);
print!("1\n");

        let mut content_string: String = String::new();
print!("1\n");
        reader.read_to_string(&mut content_string).unwrap();
print!("1\n");
        let json_object: Value = serde_json::from_str(&content_string)?;
print!("1\n");
        /* Parse JSON */
        let ip_string = format!("{}:{}", json_object["ip"].as_str().unwrap().to_string(),
        json_object["port"].as_str().unwrap().to_string());
print!("1\n");
        Ok(ip_string)
}

/* Will panic if file cannot be created in the same directory */
pub fn create_create_default_file()
{
    /* create a dummy JSON preference file */
    let json_file = r#"{
        "ip": "127.0.0.1",
        "port": 80
    }"#;
    print!("2\n");
    /* Create the file */
    let file = match File::create("./pref.json")
    {
        Ok(file_handle) => file_handle,
        Err(_err) => panic!("Check files permissions, could not write preferences file.")
    };
    print!("2\n");
    /* Convert the serde strructure to a rust string  */
    let data = serde_json::to_string(&json_file).unwrap();
    print!("2\n");
    /* prepare the writing buffer */
    let mut writer = std::io::BufWriter::new(file);
    print!("2\n");
    /* write the rust string with json in it to file system*/
    writer.write(data.as_bytes()).unwrap();
    print!("2\n");
    /* writer & file going out of scope, the file is going to be closed and dropped */
}
