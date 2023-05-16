use crate::blocks::{Block, Updatable};
use crate::world::World;

impl World {
    pub fn step(&mut self) {
        // Tick updates
        while let Some(idx) = self.tick_updatable.pop() {
            if idx.weight.update(idx, &mut self.tick_updatable) {
                self.updatable.push(idx);
            }
        }

        // End-of-tick updates
        for idx in self.updatable.drain(..) {
            if idx.weight.late_updatable(idx, &mut self.tick_updatable, self.tick_counter) {
                self.tick_updatable.extend(idx.outgoing_neighbours());
            }
        }

        self.tick_counter += 1;
    }

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &t in &self.triggers {
            let Block::Redstone(r) = &t.weight else {
                unreachable!()
            };
            r.toggle_signal();

            self.tick_updatable.extend(t.outgoing_neighbours());
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            let Block::Redstone(r) = &t.weight else {
                unreachable!()
            };
            r.toggle_signal();

            self.tick_updatable.extend(t.outgoing_neighbours());
        }
    }
}
