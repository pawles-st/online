use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub struct Generator {
    n: usize,
    rng: ThreadRng,

    std: Uniform<f64>,
    uniform: Uniform<usize>,

    harmonic_cdf: Vec<f64>,
    biharmonic_cdf: Vec<f64>,
}

impl Generator {
    pub fn new(n: usize) -> Self {
        let harmonic_max = (1..=n).rev().fold(0.0, |acc, k| acc + 1.0 / k as f64);
        let mut harmonic_cdf = (2..=n).rev().fold(vec![harmonic_max], |mut v, k| {
            v.push(*v.last().unwrap() - 1.0 / k as f64);
            v
        });
        harmonic_cdf = harmonic_cdf.into_iter().rev().map(|v| v / harmonic_max).collect();

        let biharmonic_max = (1..=n).rev().fold(0.0, |acc, k| acc + 1.0 / (k * k) as f64);
        let mut biharmonic_cdf = (2..=n).rev().fold(vec![biharmonic_max], |mut v, k| {
            v.push(*v.last().unwrap() - 1.0 / (k * k) as f64);
            v
        });
        biharmonic_cdf = biharmonic_cdf.into_iter().rev().map(|v| v / biharmonic_max).collect();
        
        Generator{
            n,
            rng: thread_rng(),
            std: Uniform::new(0.0, 1.0),
            uniform: Uniform::new(1, n + 1),
            harmonic_cdf,
            biharmonic_cdf,
        }
    }

    pub fn uniform(&mut self) -> usize {
        self.uniform.sample(&mut self.rng)
    }

    pub fn harmonic(&mut self) -> usize {
        let p = self.std.sample(&mut self.rng);
        self.harmonic_cdf.partition_point(|&v| v < p) + 1
    }

    pub fn biharmonic(&mut self) -> usize {
        let p = self.std.sample(&mut self.rng);
        self.biharmonic_cdf.partition_point(|&v| v < p) + 1
    }

    pub fn geometric(&mut self) -> usize {
        let p = self.std.sample(&mut self.rng);
        usize::min(self.n, f64::floor(-f64::log2(1.0 - p)) as usize) + 1
    }

    pub fn std(&mut self) -> f64 {
        self.std.sample(&mut self.rng)
    }
}
