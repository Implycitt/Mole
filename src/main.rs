mod client;
mod server;

use server::{ServerCommand};
use std::io::{Write, stdin, stdout};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

fn main() {
    stdout().flush().unwrap();

    let mut ip = String::new();
    stdin().read_line(&mut ip).unwrap();
    let mut ip = ip.trim().to_string();

    if ip.is_empty() {
        ip = String::from("127.0.0.1:5000");
    }

    let mut server = server::Server::new(ip.as_str());
    let (tx, rx) = mpsc::channel();

    server.listen();
    server.read_messages();
    server.handle_commands(rx);

    loop {
        commands(tx.clone());
    }
}

fn commands(sender: Sender<ServerCommand>) {
    let stdin = stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            println!("Ip: ");
            stdout().flush().unwrap();
            let mut ip = String::new();
            stdin.read_line(&mut ip).unwrap();

            let ip = ip.trim();
            sender.send(ServerCommand::Connect(ip.to_string())).unwrap();
        }
        "2" => {
            println!("Type your message: ");
            stdout().flush().unwrap();

            let mut message = String::new();
            stdin.read_line(&mut message).unwrap();

            let message = message.trim();
            sender
                .send(ServerCommand::SendMessage(message.to_string()))
                .unwrap();
        }
        _ => {
            println!("Invalid command");
        }
    }
}
