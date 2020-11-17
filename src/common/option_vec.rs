pub struct OptionVec<T> {
    data: Vec<(usize, Option<T>)>,
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

    pub fn garbage_collect(mut self) -> OptionVec<T> {
        self.data = self
            .data
            .into_iter()
            .filter_map(|cell| match cell {
                (_, None) => None,
                (_, Some(_)) => Some(cell),
            })
            .collect();
        self
    }
}
