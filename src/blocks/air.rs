use crate::blocks::{BlockTrait};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Air {}

impl BlockTrait for Air {
    fn update(
        &mut self,
        _pos: (usize, usize, usize),
        _world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        (vec![], false)
    }
}
