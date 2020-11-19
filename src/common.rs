use std::io::Read;
use std::net::TcpStream;
use std::thread;

pub mod option_vec;
pub mod roll;

pub const MESSAGE_SIZE: usize = 128;
pub const NAME_SIZE: usize = 32;

// transmission: <info><info><EOT>
// message info: <name>\0<message>\0
pub const EOT: u8 = 4;
pub const MESSAGE_INFO_SIZE: usize = MESSAGE_SIZE + NAME_SIZE + 2;
pub const MESSAGE_COUNT: usize = 32;
pub const TRANSMISSION_SIZE: usize = MESSAGE_COUNT * MESSAGE_INFO_SIZE; //about 5kb

pub const INPUT_FAIL: &str = "Failed to read input";

pub fn sleep(ms: u64) {
    thread::sleep(::std::time::Duration::from_millis(ms));
}

pub fn read_socket_data(
    socket: &mut TcpStream,
    size: usize,
    terminator: u8,
) -> Result<Vec<u8>, std::io::Error> {
    let mut buff = vec![terminator; size];

    match socket.read_exact(&mut buff) {
        Ok(_) => {
            let msg = buff
                .into_iter()
                .take_while(|&x| x != terminator)
                .collect::<Vec<_>>();
            Ok(msg)
        }
        Err(e) => Err(e),
    }
}

pub fn read_line() -> String {
    let mut buff = String::new();
    std::io::stdin().read_line(&mut buff).expect(INPUT_FAIL);
    buff
}
