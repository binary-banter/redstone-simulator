use crate::blocks::{Block, OutputPower, Updatable};
use crate::world::World;

impl World {
    pub fn step(&mut self) {
        // Tick updates
        self.tick_updatable.sort_by(|(_,y),(_,w)| y.cmp(w));
        while let Some((idx, up)) = self.tick_updatable.pop() {
            if idx.weight.update(idx, &mut self.tick_updatable, up) {
                self.updatable.push(idx);
            }
        }

        // End-of-tick updates
        for idx in self.updatable.drain(..) {
            let prev_power = idx.weight.output_power();
            idx.weight.late_update(idx, &mut self.tick_updatable, self.tick_counter);
            let next_power = idx.weight.output_power();

            match (prev_power, next_power) {
                (0, 15) => {
                    self.tick_updatable.extend(
                        idx.outgoing_neighbours().map(|n| (n, true))
                    );
                },
                (15, 0)=> {
                    self.tick_updatable.extend(
                        idx.outgoing_neighbours().map(|n| (n, false))
                    );
                },
                (prev, next) => {
                    if next > prev {
                        self.tick_updatable.extend(
                            idx.outgoing_edges().iter()
                                .filter(|e| (prev..next).contains(&e.weight))
                                .map(|n| (n.node, true))
                        )
                    } else {
                        // edge weight = 4
                        // next = 4
                        // prev = 12
                        // needs to update 4..12

                        self.tick_updatable.extend(
                            idx.outgoing_edges().iter()
                                .filter(|e| (next..prev).contains(&e.weight))
                                .map(|n| (n.node, false))
                        )
                    }
                },
            }

        //self.tick_updatable.extend(idx.outgoing_neighbours());
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

            self.tick_updatable.extend(t.outgoing_neighbours().map(|n| (n, true)));
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            let Block::Redstone(r) = &t.weight else {
                unreachable!()
            };
            r.toggle_signal();

            self.tick_updatable.extend(t.outgoing_neighbours().map(|n| (n, false)));
        }
    }
}
