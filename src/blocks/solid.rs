


use crate::blocks::{BlockTrait};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Solid {
    /// Can be 0 (off), 1 (powered) or 16 (repeater powered).
    pub signal: u8,
}

impl Solid {
    fn out_nbs(&self, p: (usize, usize, usize), world: &WorldData) -> Vec<(usize, usize, usize)> {
        world.neighbours(p)
    }
}

impl BlockTrait for Solid {


    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        let s_new = world
            .neighbours_and_facings(p)
            .into_iter()
            .map(|(n, f)| {
                let n_block = &world[n];
                n_block
                    .weak_power_dir(f)
                    .min(1)
                    .max(n_block.strong_power_dir(f))
            })
            .max()
            .unwrap();

        // if signal strength has changed, update neighbours
        if self.signal != s_new {
            self.signal = s_new;
            (self.out_nbs(p, world), false)
        } else {
            (vec![], false)
        }
    }
}
