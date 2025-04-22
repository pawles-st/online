use distributions::Generator;
use std::fs::File;
use std::fmt;
use std::io;
use std::io::BufWriter;
use std::io::Write;

mod paging;

use paging::*;

const REPS: usize = 10000;
const REQUESTS: usize = 1000;

const N: [usize; 9] = [20, 30, 40, 50, 60, 70, 80, 90, 100];
const K_RANGE: [usize; 2] = [10, 5]; // n/10...n/5

#[derive(Copy, Clone, PartialEq)]
enum CacheType {
    FIFO, FWF, LRU, LFU, RAND, RMA,
}

impl fmt::Display for CacheType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheType::FIFO => write!(f, "FIFO"),
            CacheType::FWF => write!(f, "FWF"),
            CacheType::LRU => write!(f, "LRU"),
            CacheType::LFU => write!(f, "LFU"),
            CacheType::RAND => write!(f, "RAND"),
            CacheType::RMA => write!(f, "RMA"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum DataType {
    Uniform, Harmonic, Biharmonic, Geometric,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Uniform => write!(f, "Uniform"),
            DataType::Harmonic => write!(f, "Harmonic"),
            DataType::Biharmonic => write!(f, "Biharmonic"),
            DataType::Geometric => write!(f, "Geometric"),
        }
    }
}

fn generate(g: &mut Generator, t: DataType) -> usize {
    match t {
        DataType::Uniform => g.uniform(),
        DataType::Harmonic => g.harmonic(),
        DataType::Biharmonic => g.biharmonic(),
        DataType::Geometric => g.geometric(),
    }
}

fn measure(cache_type: CacheType, data_type: DataType, n: usize, k: usize, requests: usize, reps: usize) -> Vec<f64> {
    let mut g = Generator::new(n);

    let total_cost = (0..reps).fold(vec![0; requests], |mut total_cost, _| {
        let mut cache: Box<dyn CacheStrategy<usize>> = match cache_type {
            CacheType::FIFO => Box::new(FIFO::new(k)),
            CacheType::FWF => Box::new(FWF::new(k)),
            CacheType::LRU => Box::new(LRU::new(k)),
            CacheType::LFU => Box::new(LFU::new(k)),
            CacheType::RAND => Box::new(RAND::new(k)),
            CacheType::RMA => Box::new(RMA::new(k)),
        };

        let mut compound_cost = 0;

        for i in 0..requests {
            let val = generate(&mut g, data_type);
            let new_cost = cache.access(val);
            compound_cost += new_cost;
            total_cost[i] += compound_cost;
        }

        total_cost
    });

    total_cost
        .iter()
        .map(|v| *v as f64 / reps as f64)
        .collect()
}

fn write_vec_to_file<T: fmt::Display>(vec: Vec<T>, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for value in vec {
        writeln!(writer, "{}", value)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut progress = 1;
    let no_experiments = 6 * 4;

    for cache_type in [CacheType::FIFO, CacheType::FWF, CacheType::LRU, CacheType::LFU, CacheType::RAND, CacheType::RMA] {
        for data_type in [DataType::Uniform, DataType::Harmonic, DataType::Biharmonic, DataType::Geometric] {
            print!("\rProgress: {}/{}", progress, no_experiments);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            for n in N {
                for k in (n / K_RANGE[0])..=(n / K_RANGE[1]) {
                    let cost = measure(cache_type, data_type, n, k, REQUESTS, REPS);
                    let filename = format!("results/result_{}_{}_{}_{}.txt", cache_type, data_type, n, k);
                    write_vec_to_file(cost, &filename)?
                }
            }
            progress += 1;
        }
    }

    println!();
    Ok(())
}
