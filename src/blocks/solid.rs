use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::repeater::Repeater;
use crate::blocks::torch::Torch;
use crate::blocks::{Block, BlockTrait};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Solid {
    /// Can be 0 (off), 1 (powered) or 16 (repeater powered).
    pub signal: u8,
}

impl Solid {
    fn out_nbs(&self, p: (usize, usize, usize), world: &WorldData) -> Vec<(usize, usize, usize)> {
        world.neighbours(p)
    }
}

impl BlockTrait for Solid {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        let s_new = world
            .neighbours_and_facings(p)
            .into_iter()
            .map(|(n, f)| {
                let n_block = &world[n];
                match n_block {
                    Block::Redstone(Redstone {
                        signal: 1..,
                        connections: c,
                    }) => {
                        if c[f.reverse()] {
                            1
                        } else {
                            0
                        }
                    }
                    Block::Repeater(Repeater {
                        signal: 16,
                        facing: nf,
                        ..
                    }) if f == *nf => 16,
                    Block::Solid(_)
                    | Block::Redstone(_)
                    | Block::Trigger(_)
                    | Block::Repeater(_)
                    | Block::Air
                    | Block::RedstoneBlock => 0,
                    Block::Torch(Torch { signal: s, .. }) => {
                        if *s > 0 && f == Facing::Down {
                            16
                        } else {
                            0
                        }
                    }
                }
            })
            .max()
            .unwrap_or(0);

        // if signal strength has changed, update neighbours
        if self.signal != s_new {
            self.signal = s_new;
            (self.out_nbs(p, world), false)
        } else {
            (vec![], false)
        }
    }
}
