use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::solid::Solid;
use crate::blocks::trigger::Trigger;
use crate::blocks::{Block, BlockTrait};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Repeater {
    /// Can be 0 (off) or 16 (powered).
    pub signal: u8,

    /// This is the direction of the input side.
    pub facing: Facing,

    /// This represents the number of ticks passed since a new signal was deteceted.
    pub count: u8,

    /// The repeater delay in ticks, can range from 1 to 4 inclusive.
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
        let in_nbs = world.neighbours(p).find(|(_, f)| *f == self.facing);

        // find signal strength of input
        let s_new = match in_nbs {
            None => 0,
            Some((n, _)) => match world[n] {
                Block::Solid(Solid { signal: 1.. })
                | Block::Redstone(Redstone { signal: 1.. })
                | Block::Trigger(Trigger { signal: 16 }) => 16,
                Block::Repeater(Repeater {
                    signal: 16,
                    facing: nf,
                    ..
                }) if self.facing == nf => 16,
                Block::Solid { .. }
                | Block::Redstone { .. }
                | Block::Trigger { .. }
                | Block::Repeater { .. }
                | Block::Air(_) => 0,
            },
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
