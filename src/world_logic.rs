use crate::blocks::trigger::Trigger;
use crate::blocks::{Block, BlockTrait};
use crate::world::World;
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
                Block::Air(ref mut v) => v.update(p, &self.data),
            };
            self.data[p] = block;

            tick_updatable.append(&mut updates);
            if self_update {
                self.updatable.push(p);
            }
        }
    }

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &t in &self.triggers {
            self.data[t] = Block::Trigger(Trigger { signal: 16 });
            for (n, _) in self.data.neighbours(t) {
                self.updatable.push(n);
            }
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            self.data[t] = Block::Trigger(Trigger { signal: 0 });
            for (n, _) in self.data.neighbours(t) {
                self.updatable.push(n);
            }
        }
    }
}
