use crate::blocks::trigger::Trigger;
use crate::blocks::{Block, BlockTrait, BlockTraitLate};
use crate::world::World;
use itertools::Itertools;
use std::mem;

impl World {
    pub fn step(&mut self) {
        let mut tick_updatable = mem::take(&mut self.updatable);

        while let Some(p) = tick_updatable.pop() {
            let mut block = self.data[p].clone();
            let (mut updates, self_update) = match block {
                Block::Solid(ref mut v) => v.update(p, &self.data),
                Block::Redstone(ref mut v) => v.update(p, &self.data),
                Block::Trigger(ref mut v) => v.update(p, &self.data),
                Block::Repeater(ref mut v) => v.update(p, &self.data),
                Block::Air => continue,
                Block::Torch(ref mut v) => v.update(p, &self.data),
            };
            self.data[p] = block;

            tick_updatable.append(&mut updates);
            if self_update {
                self.updatable.push(p);
            }
        }

        // perform end-of-tick updates.
        for &p in self.updatable.clone().iter().unique() {
            let mut block = self.data[p].clone();

            #[allow(clippy::single_match)]
            let mut updates = match block {
                Block::Repeater(ref mut v) => v.update_late(p, &self.data),
                Block::Torch(ref mut v) => v.update_late(p, &self.data),
                _ => vec![],
            };

            self.data[p] = block;
            self.updatable.append(&mut updates);
        }
    }

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &t in &self.triggers {
            self.data[t] = Block::Trigger(Trigger { signal: 16 });
            for n in self.data.neighbours(t) {
                self.updatable.push(n);
            }
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            self.data[t] = Block::Trigger(Trigger { signal: 0 });
            for n in self.data.neighbours(t) {
                self.updatable.push(n);
            }
        }
    }
}
