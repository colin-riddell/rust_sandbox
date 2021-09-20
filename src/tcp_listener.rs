use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_client(mut stream: TcpStream) ->std::io::Result<()> {
    // stream.write(&[1])?;
    // let mut buff = vec![]
    // let mut vec = Vec::new();

    const PAYLOAD_SIZE : usize = 2048;
    let mut buf: [u8; PAYLOAD_SIZE] = [0; PAYLOAD_SIZE];
    stream.read(&mut buf)?;
    

    let mut vec = Vec::new();

    for x in (0..PAYLOAD_SIZE)  {
        vec.push(buf[x]);     
    }
    let out  =    str::from_utf8(&vec).unwrap();
    println!("{}", &out);
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