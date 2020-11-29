use crate::common::{
    errors::*, read_line, read_socket_data, sleep, slices, EOT, MESSAGE_SIZE, NAME_SIZE,
    TRANSMISSION_SIZE,
};
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, ErrorKind, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct Message {
    name: String,
    text: String,
}

pub fn join_server(port: &str) {
    execute!(stdout(), Print(format!("Joining server at {}...\n", port)));

    let mut client = TcpStream::connect(port).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");

    // name
    'main: loop {
        {
            execute!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print("Name: ")
            );
            let mut namebuff = read_line().into_bytes();
            namebuff.resize(NAME_SIZE, 0);
            if let Err(_) = client.write_all(&namebuff) {
                execute!(stdout(), Print("Connection with server was severed"));
                break 'main;
            }
            execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
        }

        let mut message = String::new();
        let mut sendmessage = false;
        let mut messages: Vec<Message> = Vec::new();
        let mut shouldrerender = true;
        loop {
            match read_socket_data(&mut client, TRANSMISSION_SIZE, EOT) {
                Ok(buff) => {
                    let info = slices(&buff, &0);
                    shouldrerender = true;
                    messages = Vec::with_capacity(info.len() / 2);
                    for i in 0..info.len() / 2 {
                        messages.push(Message {
                            name: String::from_utf8(Vec::from(info[2 * i])).expect(UTF8_ERR),
                            text: String::from_utf8(Vec::from(info[2 * i + 1])).expect(UTF8_ERR),
                        })
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(_) => {
                    execute!(stdout(), Print("Connection with server was severed"));
                    break 'main;
                }
            }

            if shouldrerender {
                shouldrerender = false;

                execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));

                for msg in messages.iter() {
                    execute!(
                        stdout(),
                        Print(format!("{} :: {}\n", msg.name.as_str(), msg.text.as_str()))
                    );
                }
                execute!(stdout(), Print(format!("{}", message.as_str())));
            }

            if sendmessage {
                sendmessage = false;
                if message.as_str() == ":quit" {
                    break 'main;
                }
                let mut msgbuff = message.into_bytes();
                msgbuff.resize(MESSAGE_SIZE, 0);
                if let Err(_) = client.write_all(&msgbuff) {
                    execute!(stdout(), Print("Connection with server was severed"));
                    break 'main;
                }
                message = String::new();
            }

            while poll(std::time::Duration::from_secs(0)).expect(CROSSTERM_EVENT_ERR) {
                match read().expect(CROSSTERM_EVENT_ERR) {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        message.push(c);
                        shouldrerender = true;
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: KeyModifiers::SHIFT,
                    }) => {
                        message.push_str(c.to_uppercase().to_string().as_str());
                        shouldrerender = true;
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: _,
                    }) => {
                        sendmessage = true;
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: _,
                    }) => {
                        message.pop();
                        shouldrerender = true;
                    }
                    _ => (),
                }
            }

            sleep(10);
        }
    }

    execute!(stdout(), Clear(ClearType::All), Print("Closing rms"));
}
