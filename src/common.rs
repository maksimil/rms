use std::thread;

pub fn sleep(ms: u32) {
    thread::sleep(::std::time::Duration::from_millis(ms));
}
