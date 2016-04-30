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

fn handle_from_client(reader : BufReader<TcpStream>, send : Sender<String>) {
    for msg in reader.lines() {
        //println!("{}", msg.unwrap());
        send.send(msg.unwrap());
    }
}

fn handle_to_client(mut writer : TcpStream, recv : Receiver<String>) {
    for msg in recv.into_iter() {
       let _ = writer.write(msg.as_str().as_bytes());
    }
}

fn handle_messages(recv : Receiver<String>) {
    for msg in recv.into_iter() {
        println!("user: {}", msg);
    }
}

fn host(binder : &str) {
    let listener = TcpListener::bind(binder).unwrap(); //make a socket
    let (send_to_group, read_from_group) = channel(); //make a channel
    spawn(move||{ handle_messages(read_from_group); }); //start handeling the messages
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let reader = BufReader::new(stream);
                let send_to_group_copy = send_to_group.clone();
                spawn(move||{
                    handle_from_client(reader, send_to_group_copy)
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
       let _ = stream.write((msg + "\n").as_str().as_bytes());
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
