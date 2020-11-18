type Cell<T> = (usize, Option<T>);

pub struct OptionVec<T> {
    pub data: Vec<Cell<T>>,
    lastid: usize,
}

impl<T> OptionVec<T> {
    pub fn new() -> OptionVec<T> {
        OptionVec {
            data: Vec::new(),
            lastid: 0,
        }
    }

    pub fn push(&mut self, e: T) -> usize {
        self.data.push((self.lastid, Some(e)));
        self.lastid += 1;
        self.lastid
    }

    pub fn garbage_collect(&mut self) {
        let mut i = 0;
        for j in 0..self.data.len() {
            if let (_, Some(_)) = self.data[j] {
                self.data.swap(i, j);
                i += 1;
            }
        }

        self.data.truncate(i);
    }

    fn _binary_search(&self, id: usize) -> usize {
        let mut l = 0;
        let mut r = self.data.len();

        while r - l > 1 {
            let m = (l + r) / 2;
            if self.data[m].0 <= id {
                l = m;
            } else {
                r = m;
            }
        }

        return l;
    }

    pub fn get_element(&self, id: usize) -> Option<&T> {
        let i = self._binary_search(id);
        match self.data[i] {
            (uid, Some(ref e)) if uid == id => Some(e),
            _ => None,
        }
    }

    pub fn remove_element(&mut self, id: usize) {
        let i = self._binary_search(id);
        self.data[i].1 = None;
    }

    pub fn values(&self) -> Vec<&T> {
        self.data
            .iter()
            .filter_map(|cell| match cell {
                (_, None) => None,
                (_, Some(ref e)) => Some(e),
            })
            .collect()
    }
}
