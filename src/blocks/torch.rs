use crate::blocks::facing::Facing;
use crate::blocks::BlockTrait;
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Torch {
    /// Can be 0 (off) or 16 (powered).
    pub signal: u8,

    /// This is the direction of the input side.
    pub facing: Facing,

    /// This represents the number of ticks passed since a new signal was deteceted.
    pub count: u8,

    /// Next signal to be set when count reaches the repeater delay.
    pub next_signal: u8,
}

impl BlockTrait for Torch {
    fn update(
        &mut self,
        _p: (usize, usize, usize),
        _world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        todo!()
    }
}
