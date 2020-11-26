use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

#[derive(Debug)]
pub struct Header {
    key: String,
    value: String
}

impl Header {
    pub fn new(key: &str, value: &str) -> Self {
        return Self {
            key: key.to_string(),
            value: value.to_string()
        }
    }
}

#[derive(Debug)]
pub struct HTTPRequest {
    verb: String,
    route: String,
    version: String,
    headers: Vec<Header>,
    body: String // correct datatype for all body types? Should be byte array?
}

impl HTTPRequest {

    /**
     * HTTPRequest "constructor". Borrows ownership of the TcpStream object
     * splits up parses and decodes TcpStream into an HTTP request
     * Returns a new HTTPRequst object
     * HTTPRequest object is not gaurenteed to be valid at this stage. Use validate() 
     * to validate the request once it's been created.
     * */
    pub fn new(mut stream: &TcpStream) -> Self {
        const PAYLOAD_SIZE : usize = 2048;
        // read the TcpStream bytes into the empty buffer
        let mut buf: [u8; PAYLOAD_SIZE] = [0; PAYLOAD_SIZE];
        stream.read(&mut buf);
        
        // Move the buffer over to a vector byte-by-byte
        let mut buffer = Vec::new();
        for x in 0..PAYLOAD_SIZE  {
            if buf[x] != 0 {
                buffer.push(buf[x]);     
            }
        }

        // Convert the bytes so a String
        let out  =  str::from_utf8(&buffer).unwrap();

        // Split the string into it's separate lines
        let lines : Vec<&str> = out.split("\r\n").collect();
   

        /*
            * Loop over all the lines in the request
            * Split the first line into verb, route, version
            * Subsiquent lines ( >0) are Headers up until the empty line ""
            * For each header ": " to get key, value and create Header objects with them
            * Find the empty line "" and after that there's the body
        */
        let mut verb = "";
        let mut route = "";
        let mut version = "";
        let mut header;
        let mut headers : Vec<Header> = Vec::new();
        let mut empty_line_index = 0;
        for (i, line) in lines.iter().enumerate() {
            println!("line: {} on line index {} ", line, i);
            if i == 0 {
                // Split the first line into verb, route vers
                // Note some browsers don't sent URI for home page /
                let request_line:Vec<&str> = lines[0].split(" ").collect();
                verb = request_line[0];
                route = request_line[1];
                version = request_line[2];
            } else if i >0 {
                if lines[i] != "" {
                    let v: Vec<&str> = line.split(": ").collect();
                    header = Header::new(v[0], v[1]);
                    headers.push(header);
                } else {
                    empty_line_index = i;
                    break;
                }
               
            }
        }      
        
        return Self {
            verb: verb.to_string(),
            route: route.to_string(),
            version: version.to_string(),
            headers: headers,
            body: lines[empty_line_index + 1].to_string() //Find the empty line "" and after that there's the body
            //TODO: get rid of empty bytes from buffer
        };
        // Ok(())
    }

    pub fn get_verb(&self) -> String {
        self.verb.to_string()
    }

}