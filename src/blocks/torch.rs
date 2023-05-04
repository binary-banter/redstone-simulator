use crate::blocks::facing::Facing;
use crate::blocks::{BlockTrait, BlockTraitLate};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Torch {
    /// Can be 0 (off) or 16 (powered).
    pub signal: u8,

    /// Direction the torch faces.
    pub facing: Facing,

    /// Next signal to be set when count reaches the torch delay (2).
    pub next_signal: u8,
}


impl BlockTrait for Torch {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        let input = self.facing.back(p);

        self.next_signal = if world[input].weak_power_dir(self.facing.reverse()) > 0 {
            0
        } else {
            16
        };

        (vec![], true)
    }
}

impl BlockTraitLate for Torch {
    fn update_late(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> Vec<(usize, usize, usize)> {
        if self.signal != self.next_signal {
            self.signal = self.next_signal;
            world.neighbours(p)
        } else {
            vec![]
        }
    }
}
