pub struct Roll<T> {
    tail: usize,
    size: usize,
    data: Vec<T>,
}

impl<T> Roll<T> {
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
        self.tail = (self.tail + 1) % self.size;
    }

    pub fn values<'a>(&'a self) -> Vec<&'a T> {
        if self.data.len() == 0 {
            Vec::new()
        } else {
            let mut refs: Vec<&'a T> = Vec::with_capacity(self.data.len());
            let mut i = self.tail % self.data.len();

            while refs.len() < self.data.len() {
                refs.push(&self.data[i]);
                i = (i + 1) % self.data.len();
            }

            refs
        }
    }
}
