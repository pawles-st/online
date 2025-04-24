use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PackError {
    InvalidSize,
}

pub trait Packer<T> {
    fn pack(&mut self, item: T) -> Result<(), PackError>;
    fn no_bins(&self) -> usize;
}

pub struct NextFit {
    no_bins: usize,
    bin_space: f64,
}

impl NextFit {
    pub fn new() -> Self {
        Self {
            no_bins: 0,
            bin_space: 0.0,
        }
    }
}

impl Packer<f64> for NextFit {
    fn pack(&mut self, item: f64) -> Result<(), PackError> {
        if !(0.0..=1.0).contains(&item) {
            return Err(PackError::InvalidSize);
        }

        if self.bin_space < item {
            self.no_bins += 1;
            self.bin_space = 1.0;
        }

        self.bin_space -= item;

        Ok(())
    }

    fn no_bins(&self) -> usize {
        self.no_bins
    }
}

pub struct RandomFit {
    bins: Vec<f64>,
    rng: ThreadRng,
}

impl RandomFit {
    pub fn new() -> Self {
        Self {
            bins: Vec::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl Packer<f64> for RandomFit {
    fn pack(&mut self, item: f64) -> Result<(), PackError> {
        if !(0.0..=1.0).contains(&item) {
            return Err(PackError::InvalidSize);
        }

        let mut available_bins: Vec<&mut f64> = self.bins
            .iter_mut()
            .filter(|b| **b >= item)
            .collect();

        if let Some(bin) = available_bins.choose_mut(&mut self.rng) {
            **bin -= item;
        } else {
            self.bins.push(1.0 - item);
        }

        Ok(())
    }

    fn no_bins(&self) -> usize {
        self.bins.len()
    }
}

pub struct FirstFit {
    bins: Vec<f64>,
}

impl FirstFit {
    pub fn new() -> Self {
        Self {
            bins: Vec::new(),
        }
    }
}

impl Packer<f64> for FirstFit {
    fn pack(&mut self, item: f64) -> Result<(), PackError> {
        if !(0.0..=1.0).contains(&item) {
            return Err(PackError::InvalidSize);
        }

        let chosen_bin = self.bins
            .iter_mut()
            .find(|b| **b >= item);

        if let Some(bin) = chosen_bin {
            *bin -= item;
        } else {
            self.bins.push(1.0 - item);
        }

        Ok(())
    }

    fn no_bins(&self) -> usize {
        self.bins.len()
    }
}

pub struct BestFit {
    bins: Vec<f64>,
}

impl BestFit {
    pub fn new() -> Self {
        Self {
            bins: Vec::new(),
        }
    }
}

impl Packer<f64> for BestFit {
    fn pack(&mut self, item: f64) -> Result<(), PackError> {
        if !(0.0..=1.0).contains(&item) {
            return Err(PackError::InvalidSize);
        }

        let chosen_bin = self.bins
            .iter_mut()
            .filter(|b| **b >= item)
            .min_by(|a, b| a.total_cmp(b));

        if let Some(bin) = chosen_bin {
            *bin -= item;
        } else {
            self.bins.push(1.0 - item);
        }

        Ok(())
    }

    fn no_bins(&self) -> usize {
        self.bins.len()
    }
}

pub struct WorstFit {
    bins: Vec<f64>,
}

impl WorstFit {
    pub fn new() -> Self {
        Self {
            bins: Vec::new(),
        }
    }
}

impl Packer<f64> for WorstFit {
    fn pack(&mut self, item: f64) -> Result<(), PackError> {
        if !(0.0..=1.0).contains(&item) {
            return Err(PackError::InvalidSize);
        }

        let chosen_bin = self.bins
            .iter_mut()
            .filter(|b| **b >= item)
            .max_by(|a, b| a.partial_cmp(b).unwrap());

        if let Some(bin) = chosen_bin {
            *bin -= item;
        } else {
            self.bins.push(1.0 - item);
        }

        Ok(())
    }

    fn no_bins(&self) -> usize {
        self.bins.len()
    }
}
