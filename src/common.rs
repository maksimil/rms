use std::thread;

pub const MESSAGE_SIZE: usize = 128;
pub const NAME_SIZE: usize = 32;

// transmission: <info><info><EOT>
// message info: <name>\0<message>\0
pub const EOT: u8 = 4;
pub const MESSAGE_INFO_SIZE: usize = MESSAGE_SIZE + NAME_SIZE + 2;
pub const MESSAGE_INFO_COUNT: usize = 32;
pub const TRANSMISSION_SIZE: usize = MESSAGE_INFO_COUNT * MESSAGE_INFO_SIZE; //about 5kb

pub fn sleep(ms: u64) {
    thread::sleep(::std::time::Duration::from_millis(ms));
}
