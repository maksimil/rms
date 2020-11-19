use crate::common::{read_line, sleep, MESSAGE_SIZE, NAME_SIZE};
use crossterm::{execute, style::Print};
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

const SOCKET_SEND_ERR: &str = "Failed to send to socket";

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
        let mut namebuff = read_line();
        namebuff = String::from(namebuff.trim());
        let mut namebuff = namebuff.into_bytes();
        namebuff.resize(NAME_SIZE, 0);
        client.write_all(&namebuff).expect(SOCKET_SEND_ERR);
    }
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MESSAGE_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message recv {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MESSAGE_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        sleep(100);
    });

    println!("Write a Message:");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("bye bye!");
}
