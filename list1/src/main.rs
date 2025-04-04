use std::fmt;
use std::fs::File;
use std::io;
use std::io::{Write, BufWriter};
use distributions::Generator;

mod dynlist;

use dynlist::*;

const N: usize = 100000;
const REPS: usize = 100;
const NO_ELEMS: usize = 100;

#[derive(Copy, Clone, PartialEq)]
enum ListType {
    Simple, TP, MTF, FC,
}

impl fmt::Display for ListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListType::Simple => write!(f, "Simple"),
            ListType::TP => write!(f, "TP"),
            ListType::MTF => write!(f, "MTF"),
            ListType::FC => write!(f, "FC"),
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

fn measure(list_type: ListType, data_type: DataType, n: usize, reps: usize) -> Vec<usize> {
    let mut g = Generator::new(NO_ELEMS);

    let mut list: Box<dyn Dynlist<usize>> = match list_type {
        ListType::Simple => Box::new(SimpleList::new()),
        ListType::TP => Box::new(TPList::new()),
        ListType::MTF => Box::new(MTFList::new()),
        ListType::FC => Box::new(FCList::new()),
    };

    let total_cost = (0..reps).fold(vec![0; n], |mut total_cost, _| {
        let mut compound_cost = 0;

        for i in 0..n {
            let val = generate(&mut g, data_type);
            let new_cost = list.access(val);
            compound_cost += new_cost;
            total_cost[i] += compound_cost;
        }

        total_cost
    });

    total_cost
        .iter()
        .map(|v| v / reps)
        .collect()
}

fn write_vec_to_file(vec: Vec<usize>, filename: &str) -> std::io::Result<()> {
    // Create or open the file
    let file = File::create(filename)?;
    
    // Use BufWriter for efficient writing
    let mut writer = BufWriter::new(file);

    // Write each element to the file, one per line
    for value in vec {
        writeln!(writer, "{}", value)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    for list_type in [ListType::Simple, ListType::TP, ListType::MTF, ListType::FC] {
        for data_type in [DataType::Uniform, DataType::Harmonic, DataType::Biharmonic, DataType::Geometric] {
            let cost = measure(list_type, data_type, N, REPS);
            let filename = format!("result_{}_{}.txt", list_type, data_type);
            write_vec_to_file(cost, &filename)?
        }
    }

    Ok(())
}
