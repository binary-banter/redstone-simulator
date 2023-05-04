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

impl Torch {
    fn out_nbs(
        &self,
        (x, y, z): (usize, usize, usize),
        _world: &WorldData,
    ) -> Vec<(usize, usize, usize)> {
        vec![
            (x.wrapping_sub(1), y, z),
            (x.wrapping_add(1), y, z),
            (x, y.wrapping_sub(1), z),
            (x, y.wrapping_add(1), z),
            (x, y, z.wrapping_sub(1)),
            (x, y, z.wrapping_add(1)),
        ]
    }
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
            self.out_nbs(p, world)
        } else {
            vec![]
        }
    }
}
