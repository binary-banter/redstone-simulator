use crate::blocks::redstone::Redstone;
use crate::blocks::repeater::Repeater;
use crate::blocks::{Block, BlockTrait};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Solid {
    /// Can be 0 (off), 1 (powered) or 16 (repeater powered).
    pub signal: u8,
}

impl BlockTrait for Solid {
    fn out_nbs(&self, p: (usize, usize, usize), world: &WorldData) -> Vec<(usize, usize, usize)> {
        world.neighbours(p)
    }

    fn in_nbs(&self, (x,y,z): (usize, usize, usize), _world: &WorldData) -> Vec<(usize, usize, usize)> {
        vec![
            (x.wrapping_sub(1), y, z),
            (x.wrapping_add(1), y, z),
            (x, y.wrapping_sub(1), z),
            (x, y.wrapping_add(1), z),
            (x, y, z.wrapping_sub(1)),
            (x, y, z.wrapping_add(1)),
        ]
    }

    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        // find biggest signal strength around this block
        let s_new = self.in_nbs(p, world)
            .into_iter()
            .map(|n| {
                let n_block = &world[n];
                match n_block {
                    Block::Redstone(Redstone { signal: 1.., .. }) => 1,
                    Block::Repeater(Repeater {
                        signal: 16,
                        facing: nf,
                        ..
                    }) if nf.back(n) == p => 16,
                    Block::Solid(_)
                    | Block::Redstone(_)
                    | Block::Trigger(_)
                    | Block::Repeater(_)
                    | Block::Air => 0,
                    Block::Torch(_) => todo!(),
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
