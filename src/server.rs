use crate::common::{sleep, MESSAGE_SIZE, NAME_SIZE};
use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

pub fn start_server(port: &str) {
    println!("Starting server at {}...", port);

    let server = TcpListener::bind(port).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("Failed to initialize nonblocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || {
                // getting name
                let name = loop {
                    let mut buff = vec![0; NAME_SIZE];

                    match socket.read_exact(&mut buff) {
                        Ok(_) => {
                            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                            let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                            println!("{} joined with name {}", addr, msg);

                            break Some(msg);
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("Closing connectioin with: {}", addr);
                            break None;
                        }
                    }

                    sleep(100);
                }
                .expect("Failed to get name");
                // getting messages
                loop {
                    let mut buff = vec![0; MESSAGE_SIZE];

                    match socket.read_exact(&mut buff) {
                        Ok(_) => {
                            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                            let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                            println!("{}: {}", addr, msg);
                            tx.send(msg).expect("failed to send msg to rx");
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("Closing connection with: {}", addr);
                            break;
                        }
                    }

                    sleep(100);
                }
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut socket| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MESSAGE_SIZE, 0);

                    socket.write_all(&buff).map(|_| socket).ok()
                })
                .collect::<Vec<_>>();
        }

        sleep(100);
    }
}
