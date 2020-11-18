use crate::common::{
    option_vec::OptionVec, read_socket_data, roll::Roll, sleep, MESSAGE_COUNT, MESSAGE_SIZE,
    NAME_SIZE,
};
use std::io::{ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

struct User {
    name: String,
    socket: TcpStream,
}

struct Message {
    uid: usize,
    text: String,
}

enum ServerMessage {
    SetName(usize, String),
    SendMessage(usize, String),
    Leave(usize),
}

use ServerMessage::*;

const CLIENT_UID_ERROR: &str = "Client was not found by uid";
const RX_MESSAGE_ERROR: &str = "Failed to send message to rx";
const UTF8_ERR: &str = "Failed to convert u8 to utf8";

pub fn start_server(port: &str) {
    println!("Starting server at {}...", port);

    let server = TcpListener::bind(port).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("Failed to initialize nonblocking");

    let mut clients: OptionVec<User> = OptionVec::new();
    let mut messages: Roll<Message> = Roll::new(MESSAGE_COUNT);
    let (tx, rx) = mpsc::channel::<ServerMessage>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client on {} connected", addr);

            let tx = tx.clone();
            let uid = clients.push(User {
                name: String::with_capacity(NAME_SIZE),
                socket: socket.try_clone().expect("Failed to clone client"),
            });

            thread::spawn(move || 'thread: loop {
                {
                    // getting name
                    loop {
                        match read_socket_data(&mut socket, NAME_SIZE, 0) {
                            Ok(buff) => {
                                tx.send(SetName(uid, String::from_utf8(buff).expect(UTF8_ERR)))
                                    .expect(RX_MESSAGE_ERROR);
                                break;
                            }
                            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                            Err(_) => {
                                tx.send(Leave(uid)).expect(RX_MESSAGE_ERROR);
                                break 'thread;
                            }
                        }

                        sleep(100);
                    }
                    // getting messages
                    loop {
                        match read_socket_data(&mut socket, MESSAGE_SIZE, 0) {
                            Ok(buff) => {
                                tx.send(SendMessage(uid, String::from_utf8(buff).expect(UTF8_ERR)))
                                    .expect(RX_MESSAGE_ERROR);
                            }
                            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                            Err(_) => {
                                tx.send(Leave(uid)).expect(RX_MESSAGE_ERROR);
                                break 'thread;
                            }
                        }

                        sleep(100);
                    }
                }
            });
        }

        let mut shouldupdate = false;

        for message in rx.try_iter() {
            shouldupdate = true;
            match message {
                SetName(id, name) => {
                    println!("Name set to {}", name);
                    clients.get_element_mut(id).expect(CLIENT_UID_ERROR).name = name;
                }
                SendMessage(uid, text) => {
                    println!(
                        "{}: {}",
                        clients.get_element(uid).expect(CLIENT_UID_ERROR).name,
                        text
                    );
                    messages.push(Message { uid, text });
                }
                Leave(id) => {
                    println!(
                        "Closing connection with {}",
                        clients.remove_element(id).expect(CLIENT_UID_ERROR).name
                    );
                    clients.garbage_collect();
                }
            }
        }
        if shouldupdate {
            // boilerplate
            let buff = vec![1; MESSAGE_SIZE];
            clients.foreach(|user| match user.socket.write_all(&buff) {
                Ok(_) => true,
                Err(_) => false,
            });
        }
        sleep(100);
    }
}
