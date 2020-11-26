use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use crate::requests;

fn handle_client(mut stream: TcpStream) ->std::io::Result<()> {
    let request = requests::HTTPRequest::new(&stream);
    
    println!(" {:?}", request);
    println!("VERB IS: {}", request.get_verb());

    let response_line = String::from("HTTP/1.1 200 OK\r\n\r\nRequest received!");
    /* Http response takes teh form of:
    * HTTP/1.1 200 OK\r\n
    * \r\n
    * Payload data for response message here
    */

    let result = stream.write(response_line.as_bytes());
    Ok(())
}

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}