use distributions::Generator;
use std::fs::File;
use std::fmt;
use std::io;
use std::io::BufWriter;
use std::io::Write;

mod graphs;
mod migration;

use graphs::*;
use migration::*;

const REPS: usize = 100;
const NO_REQUESTS: usize = 65536;

#[derive(Copy, Clone, PartialEq)]
enum GraphType {
    Torus3D, Hypercube,
}

impl fmt::Display for GraphType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphType::Torus3D => write!(f, "Torus3D"),
            GraphType::Hypercube => write!(f, "Hypercube"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum AlgorithmType {
    MoveToMin, CoinFlip,
}

impl fmt::Display for AlgorithmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgorithmType::MoveToMin => write!(f, "MoveToMin"),
            AlgorithmType::CoinFlip => write!(f, "CoinFlip"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum DataType {
    Uniform, Harmonic, Biharmonic,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Uniform => write!(f, "Uniform"),
            DataType::Harmonic => write!(f, "Harmonic"),
            DataType::Biharmonic => write!(f, "Biharmonic"),
        }
    }
}

fn generate(g: &mut Generator, t: DataType) -> usize {
    match t {
        DataType::Uniform => g.uniform(),
        DataType::Harmonic => g.harmonic(),
        DataType::Biharmonic => g.biharmonic(),
    }
}

fn measure(algorithm_type: AlgorithmType, graph_type: GraphType, data_type: DataType, d: usize, requests: usize, reps: usize) -> Vec<f64> {
    let mut g = Generator::new(64);

    let total_cost = (0..reps).fold(vec![0; requests], |mut total_cost, _| {
        let graph: Box<dyn MetricGraph> = match graph_type {
            GraphType::Torus3D => Box::new(Torus::<3>::new(4)),
            GraphType::Hypercube => Box::new(Torus::<6>::new(2)),
        };

        let mut algorithm: Box<dyn PageMigration> = match algorithm_type {
            AlgorithmType::MoveToMin => Box::new(MoveToMin::new(graph, 0, d)),
            AlgorithmType::CoinFlip => Box::new(CoinFlip::new(graph, 0, d)),
        };

        let mut compound_cost = 0;
        for i in 0..requests {
            let source = generate(&mut g, data_type) - 1;
            compound_cost += algorithm.read(source).unwrap();
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
    for algorithm_type in [AlgorithmType::MoveToMin, AlgorithmType::CoinFlip] {
        for graph_type in [GraphType::Torus3D, GraphType::Hypercube] {
            for data_type in [DataType::Uniform, DataType::Harmonic, DataType::Biharmonic] {
                for d in [2, 16, 128, 2048] {
                    println!("{}, {}, {}, {}", algorithm_type, graph_type, data_type, d);
                    let cost = measure(algorithm_type, graph_type, data_type, d, NO_REQUESTS, REPS);
                    let filename = format!("results/result_{}_{}_{}_{}.txt", algorithm_type, graph_type, data_type, d);
                    write_vec_to_file(cost, &filename)?

                }
            }
        }
    }

    Ok(())
}


