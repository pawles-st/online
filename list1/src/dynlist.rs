use std::fmt;

pub trait Dynlist<T> {
    fn access(&mut self, t: T) -> usize;
}

macro_rules! impl_list {
    ($name: ident) => {
        impl<T> $name<T> {
            pub fn new() -> Self {
                Self{v: Vec::new()}
            }
        }

        impl<T: std::fmt::Debug> fmt::Display for $name<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.v)
            }
        }
    }
}

pub struct SimpleList<T> {
    v: Vec<T>,
}

impl_list!(SimpleList);

impl<T> Dynlist<T> for SimpleList<T>
where T: Eq
{
    fn access(&mut self, t: T) -> usize {
        match self.v.iter().position(|v| *v == t) {
            Some(i) => {
                i + 1
            }
            None => {
                self.v.push(t);
                self.v.len()
            }
        }
    }
}

pub struct MTFList<T> {
    v: Vec<T>,
}

impl_list!(MTFList);

impl<T> Dynlist<T> for MTFList<T>
where T: Eq
{
    fn access(&mut self, t: T) -> usize {
        match self.v.iter().position(|v| *v == t) {
            Some(i) => {
                let item = self.v.remove(i);
                self.v.insert(0, item);
                i + 1
            }
            None => {
                self.v.push(t);
                self.v.len()
            }
        }
    }
}

pub struct TPList<T> {
    v: Vec<T>,
}

impl_list!(TPList);

impl<T> Dynlist<T> for TPList<T>
where T: Eq
{
    fn access(&mut self, t: T) -> usize {
        match self.v.iter().position(|v| *v == t) {
            Some(i) if i > 0 => {
                self.v.swap(i - 1, i);
                i + 1
            }
            Some(i) => i + 1,
            None => {
                self.v.push(t);
                self.v.len()
            }
        }
    }
}

pub struct FCList<T> {
    v: Vec<(T, usize)>,
}

impl_list!(FCList);

impl<T> Dynlist<T> for FCList<T>
where T: Eq
{
    fn access(&mut self, t: T) -> usize {
        match self.v.iter().position(|v| (*v).0 == t) {
            Some(i) => {
                self.v[i].1 += 1;
                let pos = self.v[..i]
                    .iter()
                    .position(|(_, count)| *count < self.v[i].1);
                if let Some(j) = pos {
                    let item = self.v.remove(i);
                    self.v.insert(j, item);
                }
                i + 1
            }
            None => {
                self.v.push((t, 1));
                self.v.len()
            }
        }
    }
}
