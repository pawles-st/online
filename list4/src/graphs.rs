#[derive(Debug, Clone, Copy)]
pub enum IndexErr {
    InvalidIndex,
}

fn digits_in_base(mut n: usize, b: usize, l: u8) -> Vec<usize> {
    let mut digits = (0..l).fold(Vec::new(), |mut digits, _| {
        digits.push(n % b);
        n /= b;
        digits
    });
    digits.reverse();

    digits
}

pub trait MetricGraph {
    fn distance(&self, x: usize, y: usize) -> Result<usize, IndexErr>;
    fn size(&self) -> usize;
}

pub struct Torus<const D: u8> {
    length: usize,
}

impl<const D: u8> Torus<D> {
    pub fn new(length: usize) -> Self {
        Self{ length }
    }
}

impl<const D: u8> MetricGraph for Torus<D> {
    fn distance(&self, x: usize, y: usize) -> Result<usize, IndexErr> {
        if x >= self.length.pow(D as u32) || y >= self.length.pow(D as u32) {
            return Err(IndexErr::InvalidIndex);
        }

        let x_coords = digits_in_base(x, self.length, D);
        let y_coords = digits_in_base(y, self.length, D);

        let distance = x_coords
            .iter()
            .zip(y_coords.iter())
            .fold(0, |acc, (&a, &b)| {
                let d1 = (a as isize - b as isize).rem_euclid(self.length as isize) as usize;
                let d2 = (b as isize - a as isize).rem_euclid(self.length as isize) as usize;
                acc + usize::min(d1, d2)
        });

        Ok(distance)
    }

    fn size(&self) -> usize {
        return self.length.pow(D as u32);
    }
}
