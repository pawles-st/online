use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::thread_rng;

use crate::graphs::MetricGraph;
use crate::graphs::IndexErr;

pub trait PageMigration {
    fn read(&mut self, source: usize) -> Result<usize, IndexErr>;
}

pub struct MoveToMin {
    graph: Box<dyn MetricGraph>,
    page: usize,
    d: usize,
    buffer: Vec<usize>,
}

impl MoveToMin {
    pub fn new(graph: Box<dyn MetricGraph>, page: usize, d: usize) -> Self {
        Self {
            graph,
            page,
            d,
            buffer: Vec::with_capacity(d),
        }
    }
    
    fn migrate(&mut self, target: usize) -> Result<usize, IndexErr> {
        let cost = self.d * self.graph.distance(self.page, target)?;
        self.page = target;
        Ok(cost)
    }

    fn find_min(&self) -> usize {
        (0..self.graph.size())
            .map(|candidate| {
                (candidate, self.buffer
                    .iter()
                    .map(|&src| self.graph.distance(candidate, src).unwrap())
                    .sum::<usize>()
                )
            })
            .min_by_key(|&(_, total_cost)| total_cost)
            .map(|(best_node, _)| best_node)
            .unwrap()
    }
}

impl PageMigration for MoveToMin {
    fn read(&mut self, source: usize) -> Result<usize, IndexErr> {
        let mut cost = self.graph.distance(self.page, source)?;
        
        self.buffer.push(source);

        if self.buffer.len() == self.d {
            let best = self.find_min();
            cost += self.migrate(best)?;
            self.buffer.clear();
        }

        Ok(cost)
    }
}

pub struct CoinFlip {
    graph: Box<dyn MetricGraph>,
    page: usize,
    d: usize,
    rng: ThreadRng,
    uniform: Uniform<f64>,
}

impl CoinFlip {
    pub fn new(graph: Box<dyn MetricGraph>, page: usize, d: usize) -> Self {
        Self {
            graph,
            page,
            d,
            rng: thread_rng(),
            uniform: Uniform::new(0.0, 1.0),
        }
    }
    
    fn migrate(&mut self, target: usize) -> Result<usize, IndexErr> {
        let cost = self.d * self.graph.distance(self.page, target)?;
        self.page = target;
        Ok(cost)
    }
}

impl PageMigration for CoinFlip {
    fn read(&mut self, source: usize) -> Result<usize, IndexErr> {
        let mut cost = self.graph.distance(self.page, source)?;

        if self.uniform.sample(&mut self.rng) < 1.0 / (2.0 * self.d as f64) {
            cost += self.migrate(source)?;
        }

        Ok(cost)
    }
}
