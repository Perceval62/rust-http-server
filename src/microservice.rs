use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::net::SocketAddr;

mod config;

/*  */
/* This module gives a bunch of methods to help redirect the GET/POST to external APIs */

pub fn parse_microserve_object() -> Result<config::Microservice, ()>
{
    let output_buffer: config::Microservice = Vec::new(config::Microservice);



    return output_buffer;
}

fn

pub fn redirect_request() -> Result<(), ()>
{

}
