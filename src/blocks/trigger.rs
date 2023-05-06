use crate::blocks::BlockTrait;
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Trigger {
    /// Can be 0 (off) or 16 (triggered).
    pub powered: bool,
}

impl Trigger {
    pub fn output_signal(&self) -> u8 {
        if self.powered {
            15
        } else {
            0
        }
    }
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
