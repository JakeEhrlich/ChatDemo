use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::io::{stdin, BufReader, BufRead, Write};
use std::thread::{spawn};
use std::sync::{Arc, Mutex};

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
        let _ = send.send(msg.unwrap());
    }
}

fn handle_to_client(mut writer : TcpStream, recv : Receiver<String>) {
    for msg in recv.into_iter() {
       println!("client was sent: '{}'", msg);
       let _ = writer.write((msg + "\n").as_str().as_bytes());

    }
}

fn handle_messages(recv : Receiver<String>, client_chans : Arc<Mutex<Vec<Sender<String>>>>) {
    for msg in recv.into_iter() {
        for client in &*(client_chans.lock().unwrap()) {
            let _ = client.send(msg.clone());
        }
    }
}

fn host(binder : &str) {
    let client_chans : Arc<Mutex<Vec<Sender<String>>>> = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind(binder).unwrap(); //make a socket
    let (send_to_group, read_from_group) = channel(); //make a channel
    println!("Host started on {}", binder);
    println!("Awaiting client connections...");
    let client_chans_clone = client_chans.clone();
    spawn(move||{ handle_messages(read_from_group, client_chans_clone); }); //start handeling the messages
    for stream in listener.incoming() {
        println!("A new client has connected...");
        match stream {
            Ok(stream) => {
                let writer = stream.try_clone().unwrap();
                let reader = BufReader::new(stream);
                let send_to_group_copy = send_to_group.clone();
                let (send_to_client, read_for_client) = channel();
                client_chans.lock().unwrap().push(send_to_client);
                spawn(move||{
                    handle_from_client(reader, send_to_group_copy)
                });
                spawn(move||{
                    handle_to_client(writer, read_for_client)
                });
            }
            Err(_) => {
                println!("Somthing went wrong!");
            }
        }
    }
}

fn client(binder : &str, name : &str) {
    //open a socket
    let mut write_stream = TcpStream::connect(binder).unwrap();
    println!("Connected! Welcome, {}!", name);
    let read_buf = BufReader::new(write_stream.try_clone().unwrap());
    //loop though lines
    spawn(move|| {
        for msg in read_buf.lines() {
            println!("{}", msg.unwrap());
        }
    });
    //loop until the user quits
    loop {
       let msg = get_line_std();
       if msg == ":q" {
           return;
       }
       let final_msg = name.to_string() + ": " + msg.as_str();
       let _ = write_stream.write((final_msg + "\n").as_str().as_bytes());
    }
}

fn main() {
    loop {
        println!("Would you like to start a <client> or <host>:");
        let cmd = get_line_std();
        match cmd.as_str() {
            "host" => {
                println!("Enter IP:Port ");
                let binder = get_line_std();
                host(binder.as_str());
            }
            "client" => {
                println!("Please enter your name:");
                let name = get_line_std();
                println!("Please enter the IP address to connect to:");
                let binder = get_line_std();
                client(binder.as_str(), name.as_str());
            }
            _ => {
                println!("cmd not found")
            }
        }
    }
}
