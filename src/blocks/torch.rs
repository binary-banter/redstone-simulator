use crate::blocks::facing::Facing;
use crate::blocks::BlockTrait;
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Torch {
    /// Can be 0 (off) or 16 (powered).
    pub signal: u8,

    /// Direction the torch faces.
    pub facing: Facing,

    /// Number of ticks passed since a new input signal was detected.
    pub count: u8,

    /// Next signal to be set when count reaches the torch delay (2).
    pub next_signal: u8,
}

impl BlockTrait for Torch {
    fn out_nbs(&self, p: (usize, usize, usize), world: &WorldData) -> Vec<(usize, usize, usize)> {
        todo!()
    }

    fn update(
        &mut self,
        _p: (usize, usize, usize),
        _world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        todo!()
    }
}
