use distributions::Generator;
use std::fs::File;
use std::fmt;
use std::io;
use std::io::BufWriter;
use std::io::Write;

mod bin_packing;

use bin_packing::*;

const REPS: usize = 10000;
const NO_ITEMS: usize = 1000;

#[derive(Copy, Clone, PartialEq)]
enum PackerType {
    NF, RF, FF, BF, WF
}

impl fmt::Display for PackerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackerType::NF => write!(f, "NF"),
            PackerType::RF => write!(f, "RF"),
            PackerType::FF => write!(f, "FF"),
            PackerType::BF => write!(f, "BF"),
            PackerType::WF => write!(f, "WF"),
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

fn measure(packer_type: PackerType, data_type: DataType, n: usize, reps: usize) -> Vec<f64> {
    let mut g = Generator::new(10);

    let total_competitiveness = (0..reps).fold(vec![0.0; n], |mut total_competitiveness, _| {
        let mut packer: Box<dyn Packer<f64>> = match packer_type {
            PackerType::NF => Box::new(NextFit::new()),
            PackerType::RF => Box::new(RandomFit::new()),
            PackerType::FF => Box::new(FirstFit::new()),
            PackerType::BF => Box::new(BestFit::new()),
            PackerType::WF => Box::new(WorstFit::new()),
        };

        let mut item = 0.0;
        let mut items_remaining = 0;
        let mut item_sum = 0.0;
        for total_competitiveness_i in total_competitiveness.iter_mut() {
            if items_remaining == 0 {
                items_remaining = generate(&mut g, data_type);
                item = g.std();
            }
            item_sum += item;
            items_remaining -= 1;
            packer.pack(item).unwrap();
            *total_competitiveness_i += packer.no_bins() as f64 / item_sum.ceil();
        }

        total_competitiveness
    });

    total_competitiveness
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
    for packer_type in [PackerType::NF, PackerType::RF, PackerType::FF, PackerType::BF, PackerType::WF] {
        for data_type in [DataType::Uniform, DataType::Harmonic, DataType::Biharmonic, DataType::Geometric] {
            println!("{}, {}", packer_type, data_type);
            let cost = measure(packer_type, data_type, NO_ITEMS, REPS);
            let filename = format!("results/result_{}_{}.txt", packer_type, data_type);
            write_vec_to_file(cost, &filename)?
        }
    }

    Ok(())
}

