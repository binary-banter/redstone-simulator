use std::collections::VecDeque;
use petgraph::graph::NodeIndex;
use petgraph::Incoming;
use petgraph::prelude::*;
use crate::blocks::{Edge, OutputPower, Updatable};
use crate::blocks::repeater::Repeater;
use crate::blocks::torch::Torch;
use crate::world::RedGraph;

#[derive(Clone, Debug)]
pub struct Telescope {
    delay: u8,

    input_last: bool,
    input: bool,
    powered: bool,
    powered_queue: VecDeque<usize>,

    last_update: usize,
}

impl Telescope {
    pub fn from_torch(t: &Torch) -> Self {
        Telescope {
            delay: 1,
            input_last: t.output_power() == 0,
            input: t.output_power() == 0,
            powered: t.output_power() > 0,
            powered_queue: VecDeque::new(),
            last_update: usize::MAX,
        }
    }

    pub fn from_repeater(r: &Repeater) -> Self {
        Telescope {
            delay: 1,
            input_last: r.output_power() > 0,
            input: r.output_power() > 0,
            powered: r.output_power() > 0,
            powered_queue: VecDeque::new(),
            last_update: usize::MAX,
        }
    }

    pub fn combine(&self, other: &Self) -> Self {
        Telescope {
            delay: self.delay + other.delay,
            input_last: self.input_last,
            input: self.input,
            powered: other.powered,
            powered_queue: VecDeque::new(),
            last_update: usize::MAX,
        }
    }
}

impl OutputPower for Telescope {
    fn output_power(&self) -> u8 {
        if self.powered {
            15
        } else {
            0
        }
    }
}

impl Updatable for Telescope {
    fn update(&mut self, idx: NodeIndex, _tick_updatable: &mut VecDeque<NodeIndex>, blocks: &RedGraph) -> bool {
        self.input = blocks
            .edges_directed(idx, Incoming)
            .any(|edge| match edge.weight() {
                Edge::Rear(s) => blocks[edge.source()].output_power().saturating_sub(*s) > 0,
                Edge::Side(_) => unreachable!(),
            });

        self.input != self.input_last || !self.powered_queue.is_empty()
    }

    fn late_updatable(&mut self, idx: NodeIndex, updatable: &mut VecDeque<NodeIndex>, tick_counter: usize) -> bool {
        if tick_counter == self.last_update {
            return false;
        }
        self.last_update = tick_counter;

        if self.input != self.input_last {
            self.powered_queue.push_back(tick_counter + self.delay as usize - 1);
            self.input_last = self.input;
        }

        let Some(&crossover_tick) = self.powered_queue.get(0) else {
            return false
        };

        updatable.push_back(idx);

        if tick_counter == crossover_tick {
            self.powered_queue.pop_front();
            self.powered = !self.powered;
            true
        } else {
            false
        }
    }
}