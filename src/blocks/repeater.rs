use crate::blocks::facing::Facing;
use crate::blocks::{BlockTrait, BlockTraitLate};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Repeater {
    /// Can be 0 (off) or 16 (powered).
    pub powered: bool,

    /// Direction the input side of the repeater faces.
    pub facing: Facing,

    /// Number of ticks passed since a new input signal was detected.
    pub count: u8,

    /// Repeater delay in ticks, can range from 1 to 4 inclusive.
    pub delay: u8,

    /// Next signal to be set when count reaches the repeater delay.
    pub next_powered: bool,
}

impl Repeater {
    pub fn output_signal(&self, f: Facing) -> u8 {
        if f == self.facing && self.powered {
            15
        } else {
            0
        }
    }
}

impl BlockTrait for Repeater {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
        _updates: &mut Vec<(usize, usize, usize)>,
    ) -> bool {
        // find signal strength of input
        let s_new = world[self.facing.front(p)].output_power(self.facing) > 0;

        // if signal strength has changed, update neighbours
        match (s_new, self.next_powered == s_new, self.count == 0) {
            // Signal changed upwards: update next signal and reset count.
            (true, false, _) => {
                self.next_powered = s_new;
                self.count = 0;
            }

            // Signal changed downward, and is not propagating already: update next signal.
            (false, false, true) => {
                self.next_powered = s_new;
            }

            // Other cases.
            (_, _, _) => {}
        };

        self.powered != self.next_powered
    }
}

impl BlockTraitLate for Repeater {
    fn update_late(
        &mut self,
        p: (usize, usize, usize),
        _world: &WorldData,
        updates: &mut Vec<(usize, usize, usize)>,
    ) {
        self.count += 1;
        if self.count == self.delay {
            self.powered = self.next_powered;
            self.count = 0;

            updates.push(self.facing.back(p))
        }
    }
}
