use crate::common::{
    errors::*, read_line, read_socket_data, sleep, slices, EOT, MESSAGE_SIZE, NAME_SIZE,
    TRANSMISSION_SIZE,
};
use crossterm::{execute, style::Print};
use std::io::{ErrorKind, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self};
use std::thread;

enum ThreadMesssage {
    SendMessage(String),
}
use ThreadMesssage::*;

#[derive(Debug)]
struct Message {
    name: String,
    text: String,
}

pub fn join_server(port: &str) {
    let mut stdout = std::io::stdout();

    execute!(stdout, Print(format!("Joining server at {}...\n", port)));

    let mut client = TcpStream::connect(port).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");

    // name
    {
        execute!(stdout, Print("Name: "));
        let mut namebuff = read_line().into_bytes();
        namebuff.resize(NAME_SIZE, 0);
        client.write_all(&namebuff).expect(RX_MESSAGE_ERROR);
    }
    let (tx, rx) = mpsc::channel::<ThreadMesssage>();
    let (txl, rxl) = mpsc::channel::<()>();

    // socket
    {
        let txl = txl.clone();
        thread::spawn(move || 'thread: loop {
            let mut stdout = std::io::stdout();
            loop {
                match read_socket_data(&mut client, TRANSMISSION_SIZE, EOT) {
                    Ok(buff) => {
                        let info = slices(&buff, &0);
                        let mut messages: Vec<Message> = Vec::with_capacity(info.len() / 2);
                        for i in 0..info.len() / 2 {
                            messages.push(Message {
                                name: String::from_utf8(Vec::from(info[2 * i])).expect(UTF8_ERR),
                                text: String::from_utf8(Vec::from(info[2 * i + 1]))
                                    .expect(UTF8_ERR),
                            })
                        }
                        println!("{:?}", messages);
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                    Err(_) => {
                        execute!(stdout, Print("Connection with server was severed"));
                        break 'thread;
                    }
                }
                for msg in rx.try_iter() {
                    match msg {
                        SendMessage(contents) => {
                            let mut buff = contents.into_bytes();
                            buff.resize(MESSAGE_SIZE, 0);
                            if let Err(_) = client.write_all(&buff) {
                                txl.send(()).expect(RX_MESSAGE_ERROR);
                            }
                        }
                    }
                }

                sleep(100);
            }
        });
    }

    // user input
    {
        // let txl = txl.clone();
        thread::spawn(move || loop {
            let msg = read_line();
            if msg.as_str() == ":quit" {
                txl.send(()).expect(RX_MESSAGE_ERROR);
            }
            tx.send(SendMessage(msg)).expect(RX_MESSAGE_ERROR);
        });
    }

    rxl.recv().expect("Failed to recieve leave message");

    execute!(stdout, Print("Closing rms"));
}
