use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::io::{stdin, BufReader, BufRead, Write};
use std::thread::{spawn};
use std::sync::{Arc, Mutex};
use std::error::{Error};

fn get_line_std() -> String {
    let mut out : String = "".to_string();
    let _ = stdin().read_line(&mut out);
    return out.trim_right_matches(|c| c == '\n' || c == '\r').to_string();
}

fn host(binder : &str) {
    let listener = TcpListener::bind(binder).unwrap(); //make a socket
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("someone connected!");
            }
            Err(_) => {
                println!("somthing went wrong!");
            }
        }
    }
}

fn main() {
    loop {
        let cmd = get_line_std();
        match cmd.as_str() {
            "host" => {
                let binder = get_line_std();
                host(binder.as_str());
            }
            "client" => {
                let binder = get_line_std();
                println!("you want to connect to {}", binder);
            }
            _ => {
                println!("cmd not found")
            }
        }
    }
}
