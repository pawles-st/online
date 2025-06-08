use distributions::Generator;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;

mod allocation;
use allocation::*;

const REPS: usize = 100;
const NO_REQUESTS: usize = 65536;

fn measure(d: usize, p: f64, requests: usize, reps: usize) -> Vec<(f64, f64)> {
    let mut g = Generator::new(64);

    let stats = (0..reps).fold(vec![(0, 1); requests], |mut stats, _| {
        let mut algorithm = Count::new(64, 0, d);

        let mut compound_cost = 0;
        for i in 0..requests {
            let source = g.uniform() - 1;
            if g.bernoulli(p) == 1 {
                compound_cost += algorithm.write(source).unwrap();
            } else {
                compound_cost += algorithm.read(source).unwrap();
            }
            stats[i].0 += compound_cost;
            stats[i].1 += algorithm.no_pages();
        }

        stats
    });

    stats
        .iter()
        .map(|v| ((v.0 as f64 / reps as f64), (v.1 as f64 / reps as f64)))
        .collect()
}

fn write_vec_to_file(vec: Vec<(f64, f64)>, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for value in vec {
        writeln!(writer, "{};{}", value.0, value.1)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    for d in [16, 32, 64, 128, 256] {
        for p in [0.01, 0.02, 0.05, 0.1, 0.2, 0.5] {
            println!("{}, {}", d, p);
            let cost = measure(d, p, NO_REQUESTS, REPS);
            let filename = format!("results/result_{}_{}.txt", d, p);
            write_vec_to_file(cost, &filename)?;
        }
    }

    Ok(())
}
