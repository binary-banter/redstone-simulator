use crate::blocks::BlockTrait;
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Trigger {
    /// Can be 0 (off) or 16 (triggered).
    pub signal: u8,
}

impl BlockTrait for Trigger {
    fn out_nbs(&self, (x,y,z): (usize, usize, usize), _world: &WorldData) -> Vec<(usize, usize, usize)> {
        vec![
            (x.wrapping_sub(1), y, z),
            (x.wrapping_add(1), y, z),
            (x, y.wrapping_sub(1), z),
            (x, y.wrapping_add(1), z),
            (x, y, z.wrapping_sub(1)),
            (x, y, z.wrapping_add(1)),
        ]
    }

    fn in_nbs(&self, _p: (usize, usize, usize), _world: &WorldData) -> Vec<(usize, usize, usize)> {
        vec![]
    }

    fn update(
        &mut self,
        _pos: (usize, usize, usize),
        _world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        (vec![], false)
    }
}
