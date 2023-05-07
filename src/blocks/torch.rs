use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockTrait, BlockTraitLate};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Torch {
    /// Can be 0 (off) or 16 (powered).
    pub powered: bool,

    /// Direction the torch faces.
    pub facing: Facing,
}

impl Torch {
    pub fn output_signal(&self, f: Facing) -> u8 {
        // Torch does not output to where it's hanging
        // TODO this check might not be necessary since we don't have redstone lamps?
        if self.facing == f {
            0
        } else if self.powered {
            15
        } else {
            0
        }
    }
}

impl BlockTrait for Torch {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
        _updates: &mut Vec<(usize, usize, usize)>,
    ) -> bool {
        let new_s = world[self.facing.back(p)].output_power(self.facing.reverse()) == 0;

        new_s != self.powered
    }
}

impl BlockTraitLate for Torch {
    fn update_late(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
        updates: &mut Vec<(usize, usize, usize)>,
    ) {
        self.powered = !self.powered;

        updates.extend(world.neighbours(p).filter(|&p| match world[p] {
            Block::Solid(_) => true,
            Block::Redstone(_) => true,
            Block::RedstoneBlock => false,
            Block::Trigger(_) => false,
            Block::Repeater(_) => true,
            Block::Comparator(_) => true,
            Block::Torch(_) => false,
            Block::Air => false,
        }));
    }
}
