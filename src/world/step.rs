use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, Updatable};
use crate::world::World;
use petgraph::Outgoing;

impl World {
    pub fn step(&mut self) {
        // Tick updates
        while let Some(idx) = self.tick_updatable.pop() {
            // TODO this is terribly terribly unsound
            // let block: &mut Block = unsafe { &mut *((&mut self.blocks[idx]) as *mut Block) };

            let mut block = self.blocks[idx].clone();

            if block.update(idx, &mut self.tick_updatable, &self.blocks) {
                self.updatable.push(idx);
            }

            self.blocks[idx] = block;
        }

        // End-of-tick updates
        for idx in self.updatable.drain(..) {
            if self.blocks[idx].late_updatable(idx, &mut self.tick_updatable, self.tick_counter) {
                self.tick_updatable
                    .extend(self.blocks.neighbors_directed(idx, Outgoing));
            }
        }

        self.tick_counter += 1;
    }

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &t in &self.triggers {
            self.blocks[t] = Block::Redstone(Redstone::with_signal(15));
            self.tick_updatable
                .extend(self.blocks.neighbors_directed(t, Outgoing));
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            self.blocks[t] = Block::Redstone(Redstone::with_signal(0));
            self.tick_updatable
                .extend(self.blocks.neighbors_directed(t, Outgoing));
        }
    }
}
