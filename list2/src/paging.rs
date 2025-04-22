use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use rand::rngs::ThreadRng;

pub trait CacheStrategy<T> {
    fn access(&mut self, page: T) -> usize;
}

#[derive(Debug)]
pub struct FIFO {
    cache: Vec<usize>,
    size: usize,
    index: usize,
}

impl FIFO {
    pub fn new(size: usize) -> Self {
        Self {
            cache: vec![0; size],
            size: size,
            index: 0,
        }
    }
}

impl CacheStrategy<usize> for FIFO {
    fn access(&mut self, page: usize) -> usize {
        match self.cache.iter().position(|&v| v == page) {
            Some(_) => 0,
            None => {
                self.cache[self.index] = page;
                self.index = (self.index + 1) % self.size;
                1
            }
        }
    }
}

#[derive(Debug)]
pub struct FWF {
    cache: Vec<usize>,
    size: usize,
    index: usize,
}

impl FWF {
    pub fn new(size: usize) -> Self {
        Self {
            cache: vec![0; size],
            size: size,
            index: 0,
        }
    }
}

impl CacheStrategy<usize> for FWF {
    fn access(&mut self, page: usize) -> usize {
        match self.cache.iter().position(|&v| v == page) {
            Some(_) => 0,
            None => {
                if self.index == self.size {
                    self.cache.fill(0);
                    self.index = 0;
                }
                self.cache[self.index] = page;
                self.index += 1;
                1
            }
        }
    }
}

#[derive(Debug)]
pub struct LRU {
    cache: Vec<usize>,
}

impl LRU {
    pub fn new(size: usize) -> Self {
        Self {
            cache: vec![0; size],
        }
    }
}

impl CacheStrategy<usize> for LRU {
    fn access(&mut self, page: usize) -> usize {
        match self.cache.iter().position(|&v| v == page) {
            Some(i) => {
                self.cache.remove(i);
                self.cache.push(page);
                0
            },
            None => {
                self.cache.remove(0);
                self.cache.push(page);
                1
            }
        }
    }
}

#[derive(Debug)]
pub struct LFU {
    cache: Vec<usize>,
    count: Vec<usize>,
}

impl LFU {
    pub fn new(size: usize) -> Self {
        Self {
            cache: vec![0; size],
            count: vec![0; size + 1], // 0th index is dummy for 'empty'
        }
    }
}

impl CacheStrategy<usize> for LFU {
    fn access(&mut self, page: usize) -> usize {
        if page >= self.count.len() {
            self.count.resize(page + 1, 0);
        }
        self.count[page] += 1;

        match self.cache.iter().position(|&v| v == page) {
            Some(_) => 0,
            None => {
                let min = self.cache
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, v)| self.count[**v])
                    .map(|(i, _)| i)
                    .unwrap();
                self.cache[min] = page;
                1
            }
        }
    }
}

#[derive(Debug)]
pub struct RAND {
    cache: Vec<usize>,
    size: usize,
    taken: usize,
    uniform: Uniform<usize>,
    rng: ThreadRng,
}

impl RAND {
    pub fn new(size: usize) -> Self {
        Self {
            cache: vec![0; size],
            size: size,
            taken: 0,
            uniform: Uniform::from(0..size),
            rng: rand::thread_rng(),
        }
    }
}
    
impl CacheStrategy<usize> for RAND {
    fn access(&mut self, page: usize) -> usize {
        match self.cache.iter().position(|&v| v == page) {
            Some(_) => 0,
            None => {
                if self.taken == self.size {
                    let i = self.uniform.sample(&mut self.rng);
                    self.cache[i] = page;
                } else {
                    self.cache[self.taken] = page;
                    self.taken += 1;
                }
                1
            }
        }
    }
}

#[derive(Debug)]
pub struct RMA {
    cache: Vec<(usize, bool)>,
    size: usize,
    marked: usize,
    rng: ThreadRng,
}

impl RMA {
    pub fn new(size: usize) -> Self {
        Self {
            cache: vec![(0, false); size],
            size: size,
            marked: 0,
            rng: rand::thread_rng(),
        }
    }
}

impl CacheStrategy<usize> for RMA {
    fn access(&mut self, page: usize) -> usize {
        match self.cache.iter().position(|(v, _)| *v == page) {
            Some(i) => {
                if self.cache[i].1 == false {
                    self.cache[i].1 = true;
                    self.marked += 1;
                }
                0
            }
            None => {
                if self.marked == self.size {
                    self.cache.iter_mut().for_each(|(_, marked)| *marked = false);
                    self.marked = 0;
                }
                let rand = self.rng.gen_range(0..(self.size - self.marked));
                let i = self.cache
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, marked))| *marked == false)
                    .nth(rand)
                    .map(|(i, _)| i)
                    .unwrap();
                self.cache[i] = (page, true);
                self.marked += 1;
                1
            }
        }
    }
}
