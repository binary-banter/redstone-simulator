use crate::blocks::facing::Facing;
use crate::blocks::repeater::Repeater;
use crate::blocks::solid::Solid;
use crate::blocks::trigger::Trigger;
use crate::blocks::{Block, BlockTrait};
use crate::world_data::WorldData;

#[derive(Debug, Clone, PartialEq)]
pub struct Redstone {
    /// Ranges from 0 to 15 inclusive.
    pub signal: u8,
}

impl BlockTrait for Redstone {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        let in_nbs = world.neighbours(p);
        let out_nbs = world.neighbours(p).filter(|(_, f)| *f != Facing::Up);

        // find biggest signal strength around this block
        let s_new = in_nbs
            .map(|(n, _)| {
                let n_block = &world[n];
                match n_block {
                    Block::Redstone(Redstone { signal: ns }) => ns.saturating_sub(1),
                    Block::Repeater(Repeater {
                        signal: 16,
                        facing: nf,
                    }) if nf.back(n) == p => 15,
                    Block::Trigger(Trigger { signal: 16 }) | Block::Solid(Solid { signal: 16 }) => {
                        15
                    }
                    _ => 0,
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
