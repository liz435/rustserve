use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    
    let local_host_addr  = "127.0.0.1:7878";
    
    let listener = TcpListener::bind(local_host_addr);

    match listener {
        Ok(v) => listen_and_serve(v),
        Err(err) => println!(" {err:?} error")
    }
}

fn listen_and_serve(listener: TcpListener){

    for stream in listener.incoming(){
        
        match stream {
            Ok(v) => println!("Connection established! {v:?}"),
            Err(err) => println!("Error: {err:?}")
        }
    }
} 

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];

    let data = stream.read(&mut buffer);

    match data {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("{e:?}")
    }
}