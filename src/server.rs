use crate::client::Client;
use core::panic;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

pub struct Server {
    listener: Arc<TcpListener>,
    clients: Arc<Mutex<Vec<Client>>>,
}

pub enum ServerCommand {
    Connect(String),
    SendMessage(String),
}

impl Server {
    pub fn new(ip: &str) -> Server {
        let listener_result = TcpListener::bind(ip);
        let listener = match listener_result {
            Ok(listener) => listener,
            Err(err) => {
                panic!("Can't listen on this IP address: {}, Err: {}", ip, err)
            }
        };

        println!("Listening on {}", ip);
        Server {
            listener: Arc::new(listener),
            clients: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn listen(&mut self) {
        let temp_listener = Arc::clone(&self.listener);
        let temp_clients = Arc::clone(&self.clients);

        thread::spawn(move || {
            for stream in temp_listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!(
                            "New connection {}",
                            stream.try_clone().unwrap().peer_addr().unwrap()
                        );
                        let client = Client::new(stream.try_clone().unwrap());
                        temp_clients.lock().unwrap().push(client);
                    }
                    _ => {}
                }
            }
        });
    }

    fn connect(mut clients: MutexGuard<Vec<Client>>, ip: &str) {
        let stream_result = TcpStream::connect(ip);

        match stream_result {
            Ok(stream) => {
                let client = Client::new(stream.try_clone().unwrap());
                clients.push(client);
                println!("Connected Successfully");
            }
            Err(err) => {
                println!("Can't connect to the ip: {}, Err: {}", ip, err);
            }
        }
    }

    fn send_message(mut clients: MutexGuard<Vec<Client>>, message: String) {
        if clients.is_empty() {
            println!("No clients");
            return;
        }

        for client in clients.iter_mut() {
            client.write_stream(message.as_str());
        }
    }

    pub fn read_messages(&mut self) {
        let temp_client = Arc::clone(&self.clients);
        thread::spawn(move || {
            loop {
                let mut clients = temp_client.lock().unwrap();

                for client in clients.iter_mut() {
                    let data = client.read();

                    match data {
                        None => {}
                        Some(data) => {
                            println!("\n New message: {}", data);
                        }
                    }
                }
            }
        });
    }

    pub fn handle_commands(&self, receiver: Receiver<ServerCommand>) {
        let temp_clients = Arc::clone(&self.clients);
        thread::spawn(move || {
            while let Ok(command) = receiver.recv() {
                match command {
                    ServerCommand::Connect(port) => {
                        let clients = temp_clients.lock().unwrap();
                        Server::connect(clients, port.as_str());
                    }
                    ServerCommand::SendMessage(message) => {
                        let clients = temp_clients.lock().unwrap();
                        Server::send_message(clients, message);
                    }
                }
            }
        });
    }
}
