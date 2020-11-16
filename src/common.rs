use std::thread;

pub const MESSAGE_SIZE: usize = 128;

pub fn sleep(ms: u64) {
    thread::sleep(::std::time::Duration::from_millis(ms));
}
