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
}

impl BlockTrait for Repeater {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        let in_nbs = world.neighbours(p).find(|(_, f)| *f == self.facing);
        let out_nbs = world
            .neighbours(p)
            .filter(|(_, f)| *f == self.facing.reverse());

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
                }) if self.facing == nf => 16,
                Block::Solid { .. }
                | Block::Redstone { .. }
                | Block::Trigger { .. }
                | Block::Repeater { .. }
                | Block::Air(_) => 0,
            },
        };

        // if signal strength has changed, update neighbours
        let marked_neighbours = if self.signal != s_new {
            self.signal = s_new;
            out_nbs.map(|(n, _)| n).collect()
        } else {
            vec![]
        };

        (marked_neighbours, false)
    }
}
