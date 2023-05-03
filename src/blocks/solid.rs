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
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        let in_nbs = world.neighbours(p);
        let out_nbs = world.neighbours(p);

        // find biggest signal strength around this block
        let s_new = in_nbs
            .map(|(n, _)| {
                let n_block = &world[n];
                match n_block {
                    Block::Redstone(Redstone { signal: 1.. }) => 1,
                    Block::Repeater(Repeater {
                        signal: 16,
                        facing: nf,
                        ..
                    }) if nf.back(n) == p => 16,
                    Block::Solid(_)
                    | Block::Redstone(_)
                    | Block::Trigger(_)
                    | Block::Repeater(_)
                    | Block::Air(_) => 0,
                    Block::Torch(_) => todo!(),
                }
            })
            .max()
            .unwrap_or(0);

        // if signal strength has changed, update neighbours
        let marked_neighbours = if self.signal != s_new {
            self.signal = s_new;
            out_nbs.map(|(p, _)| p).collect()
        } else {
            vec![]
        };
        (marked_neighbours, false)
    }
}
