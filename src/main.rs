use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::io::{stdin, BufReader, BufRead, Write};
use std::thread::{spawn};
use std::sync::{Arc, Mutex};
use std::error::{Error};

fn trim(s : String) -> String {
    return s.trim_right_matches(|c| c == '\n' || c == '\r').to_string();
}

fn get_line_std() -> String {
    let mut out : String = "".to_string();
    let _ = stdin().read_line(&mut out);
    return trim(out);
}

fn host(binder : &str) {
    let listener = TcpListener::bind(binder).unwrap(); //make a socket
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut reader = BufReader::new(stream);
                spawn(move||{
                    for msg in reader.lines() {
                        println!("user: {}", trim(msg.unwrap()));
                    }
                });
            }
            Err(_) => {
                println!("somthing went wrong!");
            }
        }
    }
}

fn client(binder : &str) {
    //open a socket
    let mut stream = TcpStream::connect(binder).unwrap();
    //loop until the user quits
    loop {
       let msg = get_line_std();
       if msg == ":q" {
           return;
       }
       stream.write((msg + "\n").as_str().as_bytes());
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
                client(binder.as_str());
            }
            _ => {
                println!("cmd not found")
            }
        }
    }
}
