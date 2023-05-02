use crate::blocks::{BlockTrait};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Trigger {
    /// Can be 0 (off) or 16 (triggered).
    pub signal: u8,
}

impl BlockTrait for Trigger {
    fn update(
        &mut self,
        _pos: (usize, usize, usize),
        _world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        (vec![], false)
    }
}
