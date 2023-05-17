use crate::blocks::{Block, Updatable};
use crate::world::World;

impl World {
    pub fn step(&mut self) {
        // Tick updates
        while let Some(idx) = self.tick_updatable.up.pop() {
            if idx.weight.update(idx, &mut self.tick_updatable.down, true) {
                self.updatable.push(idx);
            }
        }
        while let Some(idx) = self.tick_updatable.down.pop() {
            if idx.weight.update(idx, &mut self.tick_updatable.down, false) {
                self.updatable.push(idx);
            }
        }

        // End-of-tick updates
        for idx in self.updatable.drain(..) {
            match idx
                .weight
                .late_update(idx, &mut self.tick_updatable.down, self.tick_counter)
            {
                Some((0, 15)) => {
                    self.tick_updatable.up.extend(idx.outgoing_neighbours());
                }
                Some((15, 0)) => {
                    self.tick_updatable.down.extend(idx.outgoing_neighbours());
                }
                Some((prev, next)) => {
                    if next > prev {
                        self.tick_updatable.up.extend(
                            idx.outgoing_edges()
                                .iter()
                                .filter(|e| (prev..next).contains(&e.weight))
                                .map(|e| e.node),
                        )
                    } else {
                        self.tick_updatable.down.extend(
                            idx.outgoing_edges()
                                .iter()
                                .filter(|e| (next..prev).contains(&e.weight))
                                .map(|e| e.node),
                        )
                    }
                }
                None => {}
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

            self.tick_updatable.up.extend(t.outgoing_neighbours());
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            let Block::Redstone(r) = &t.weight else {
                unreachable!()
            };
            r.toggle_signal();

            self.tick_updatable.down.extend(t.outgoing_neighbours());
        }
    }
}
