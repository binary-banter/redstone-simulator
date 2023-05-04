use crate::blocks::facing::Facing;
use crate::blocks::{BlockTrait, BlockTraitLate};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Repeater {
    /// Can be 0 (off) or 16 (powered).
    pub signal: u8,

    /// Direction the input side of the repeater faces.
    pub facing: Facing,

    /// Number of ticks passed since a new input signal was detected.
    pub count: u8,

    /// Repeater delay in ticks, can range from 1 to 4 inclusive.
    pub delay: u8,

    /// Next signal to be set when count reaches the repeater delay.
    pub next_signal: u8,
}

impl BlockTrait for Repeater {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        // find signal strength of input
        let s_new = if world[self.facing.front(p)].weak_power_dir(self.facing) > 0 {
            16
        } else{
            0
        };

        // if signal strength has changed, update neighbours
        match (s_new, self.next_signal == s_new, self.count == 0) {
            // Signal changed upwards: update next signal and reset count.
            (16, false, _) => {
                self.next_signal = s_new;
                self.count = 0;
            }

            // Signal changed downward, and is not propagating already: update next signal.
            (0, false, true) => {
                self.next_signal = s_new;
            }

            // Other cases.
            (_, _, _) => {}
        };

        (vec![], self.signal != self.next_signal)
    }
}

impl BlockTraitLate for Repeater {
    fn update_late(
        &mut self,
        p: (usize, usize, usize),
        _world: &WorldData,
    ) -> Vec<(usize, usize, usize)> {
        self.count += 1;
        if self.count == self.delay {
            self.signal = self.next_signal;
            self.count = 0;

            vec![self.facing.back(p)]
        } else {
            vec![]
        }
    }
}
