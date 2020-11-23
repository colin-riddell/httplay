use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

pub struct HTTPRequest {
    verb: String,
    route: String,
    version: String
}

impl HTTPRequest {

    /**
     * Borrows ownership of the TcpStream 
     * splits up parses and decodes TcpStream into an HTTP request
     * Returns a new HTTPRequst object
     * */
    pub fn new(mut stream: &TcpStream) -> HTTPRequest {
        const payload_size : usize = 2048;
        let mut buf: [u8; payload_size] = [0; payload_size];
        stream.read(&mut buf);
        

        let mut vec = Vec::new();

        for x in (0..payload_size)  {
            vec.push(buf[x]);     
        }
        let out  =  str::from_utf8(&vec).unwrap();

        let v:Vec<&str> = out.split("\r\n").collect();
        let request_line:Vec<&str> = v[0].split(" ").collect();

        // Note some browsers don't sent URI for home page /
        let verb = request_line[0];
        let route = request_line[1];
        let version = request_line[2];
        println!("Verb is: {}", verb);
        println!("Route is: {}", route);
        println!("Version is: {}", version);

        println!("{}", &out);

        return HTTPRequest {
            verb: verb.to_string(),
            route: route.to_string(),
            version: version.to_string()
        };
        // Ok(())
    }

    pub fn get_verb(&self) -> String {
        self.verb.to_string()
    }

}