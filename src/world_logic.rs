use crate::blocks::{redstone_max, redstone_min, Updatable};
use crate::world::World;
use itertools::Itertools;
use petgraph::Outgoing;
use std::mem;

impl World {
    pub fn step(&mut self) {
        let mut tick_updatable = mem::take(&mut self.updatable);

        // Tick updates
        while let Some(idx) = tick_updatable.pop() {
            let mut block = self.blocks[idx].clone();

            if block.update(idx, &mut tick_updatable, &mut self.blocks) {
                self.updatable.push(idx);
            }

            self.blocks[idx] = block;
        }

        // // End-of-tick updates
        for &idx in self.updatable.clone().iter().unique() {
            let mut block = self.blocks[idx].clone();

            block.late_updatable(idx, &mut self.updatable, &mut self.blocks);

            self.blocks[idx] = block;
        }
    }

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &t in &self.triggers {
            self.blocks[t] = redstone_max();

            for n in self.blocks.neighbors_directed(t, Outgoing) {
                self.updatable.push(n);
            }
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            self.blocks[t] = redstone_min();

            for n in self.blocks.neighbors_directed(t, Outgoing) {
                self.updatable.push(n);
            }
        }
    }
}
