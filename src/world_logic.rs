use crate::blocks::{redstone_max, redstone_min, Updatable};
use crate::world::World;
use itertools::Itertools;
use petgraph::Outgoing;

impl World {
    pub fn step(&mut self) {
        // Tick updates
        while let Some(idx) = self.tick_updatable.pop_front() {
            let mut block = self.blocks[idx].clone();

            if block.update(idx, &mut self.tick_updatable, &mut self.blocks) {
                self.updatable.push_back(idx);
            }

            self.blocks[idx] = block;
        }

        // End-of-tick updates
        for idx in self.updatable.drain(..).unique() {
            if self.blocks[idx].late_updatable(idx, &mut self.tick_updatable) {
                self.tick_updatable.extend(self.blocks.neighbors_directed(idx, Outgoing));
            }
        }
    }

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &t in &self.triggers {
            self.blocks[t] = redstone_max();
            self.tick_updatable.extend(self.blocks.neighbors_directed(t, Outgoing));
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            self.blocks[t] = redstone_min();
            self.tick_updatable.extend(self.blocks.neighbors_directed(t, Outgoing));
        }
    }
}
