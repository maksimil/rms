use std::fmt::Debug;

#[derive(Debug)]
pub struct Roll<T> {
    tail: usize,
    size: usize,
    data: Vec<T>,
}

impl<T: Debug> Roll<T> {
    pub fn new(size: usize) -> Roll<T> {
        Roll {
            tail: 0,
            size,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, e: T) {
        if self.size > self.data.len() {
            self.data.push(e);
        } else {
            self.data[self.tail] = e;
        }
        self.tail = self.head();
        print!("{:?} ", self);
        println!("{:?}", self.head());
    }

    pub fn head(&self) -> usize {
        (self.tail + 1) % self.data.len()
    }
}
