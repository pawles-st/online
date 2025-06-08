use std::collections::HashSet;
use metricgraph::*;

pub trait PageAllocation {
    fn read(&mut self, source: usize) -> Result<usize, IndexErr>;
    fn write(&mut self, source: usize) -> Result<usize, IndexErr>;
}

pub struct Count {
    size: usize,
    pub pages: HashSet<usize>,
    d: usize,
    pub states: Vec<u8>,
    pub counters: Vec<usize>,
}

impl Count {
    pub fn new(size: usize, page: usize, d: usize) -> Self {
        let mut pages = HashSet::new();
        pages.insert(page);
        let mut states = vec![1; size];
        states[page] = 4;
        let counters = vec![0; size];
        Self {
            size,
            pages,
            d,
            states,
            counters,
        }
    }

    pub fn no_pages(&self) -> usize {
        self.pages.len()
    }

    fn drop_pages(&mut self) {
        for i in 0..self.size {
            if self.states[i] == 4 && self.pages.len() > 1 {
                self.states[i] = 1;
                self.pages.remove(&i);
            }
        }
    }
}

impl PageAllocation for Count {
    fn read(&mut self, source: usize) -> Result<usize, IndexErr> {
        let cost = if self.pages.contains(&source) {
            0
        } else {
            1
        };

        if self.states[source] == 1 {
            self.counters[source] += 1;
            if self.counters[source] == self.d {
                self.pages.insert(source);
                self.states[source] = 3;
                self.drop_pages();
            }
        }

        Ok(cost)
    }

    fn write(&mut self, source: usize) -> Result<usize, IndexErr> {
        let cost = if self.pages.contains(&source) {
            self.pages.len() - 1
        } else {
            self.pages.len()
        };

        for i in 0..self.size {
            if i != source && self.states[i] == 3 {
                self.counters[i] -= 1;
                if self.counters[i] == 0 {
                    self.states[i] = 4;
                }
            }
        }

        if self.states[source] == 1 {
            self.counters[source] += 1;
            if self.counters[source] == self.d {
                self.pages.insert(source);
                self.states[source] = 3;
            }
        }
        
        self.drop_pages();

        Ok(cost)
    }
}
