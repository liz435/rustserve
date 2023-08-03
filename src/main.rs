use std::fs;
use std::io::Error;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    
    let local_host_addr  = "127.0.0.1:7878";
    
    let listener = TcpListener::bind(local_host_addr);

    match listener {
        Ok(v) => listen_and_serve(v),
        Err(err) => log_err(err)
    }
}

fn listen_and_serve(listener: TcpListener){

    for stream in listener.incoming(){
        
        match stream {
            Ok(v) => handle_connection(v),
            Err(err) => log_err(err)
        }
    }
} 

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];

    let res = stream.read(&mut buffer);

    if res.is_err() {
        panic!("Could not parse TCP stream")
    }

    // this symbol [..] somehow changes the strings format, look into it
     match std::str::from_utf8(&buffer){
        Ok(v) => println!("{}", &v[..]),
        Err(e) => println!("{e:?}")
    };
    
    let content = fs::read_to_string("../index.html").unwrap();



    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                            content.len(),
                            content);

    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
 
    
}

fn log_err(err: Error){
    println!("Error: {}",err)
}