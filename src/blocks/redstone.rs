use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockTrait};
use crate::world_data::WorldData;
use std::ops::Index;

#[derive(Debug, Clone, PartialEq)]
pub struct Connections {
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}

impl Index<Facing> for Connections {
    type Output = bool;

    fn index(&self, index: Facing) -> &Self::Output {
        match index {
            Facing::North => &self.north,
            Facing::East => &self.east,
            Facing::South => &self.south,
            Facing::West => &self.west,
            Facing::Up => &false,
            Facing::Down => &true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Redstone {
    /// Ranges from 0 to 15 inclusive.
    pub signal: u8,

    /// North East South West
    pub connections: Connections,
}

impl Redstone {
    fn out_nbs(
        &self,
        (x, y, z): (usize, usize, usize),
        _world: &WorldData,
    ) -> Vec<(usize, usize, usize)> {
        vec![
            (x.wrapping_sub(1), y.wrapping_sub(1), z),
            (x.wrapping_sub(1), y, z),
            (x.wrapping_sub(1), y.wrapping_add(1), z),
            (x.wrapping_add(1), y.wrapping_sub(1), z),
            (x.wrapping_add(1), y, z),
            (x.wrapping_add(1), y.wrapping_add(1), z),
            (x, y.wrapping_sub(1), z.wrapping_sub(1)),
            (x, y, z.wrapping_sub(1)),
            (x, y.wrapping_add(1), z.wrapping_sub(1)),
            (x, y.wrapping_sub(1), z.wrapping_add(1)),
            (x, y, z.wrapping_add(1)),
            (x, y.wrapping_add(1), z.wrapping_add(1)),
            (x, y.wrapping_sub(1), z),
        ]
    }

    fn in_nbs(
        &self,
        (x, y, z): (usize, usize, usize),
        world: &WorldData,
    ) -> Vec<((usize, usize, usize), Facing)> {
        let mut in_nbs = world.neighbours_and_facings((x, y, z));

        let top = (x, y.wrapping_add(1), z);
        for f in [Facing::North, Facing::East, Facing::South, Facing::West] {
            let side = f.front((x, y, z));
            let side_down = (side.0, side.1.wrapping_sub(1), side.2);
            let side_up = (side.0, side.1.wrapping_add(1), side.2);

            match [side_down, side, side_up, top].map(|n| &world[n]) {
                [Block::Redstone(_), b, _, _] if b.is_transparent() => {
                    in_nbs.push((side_down, f));
                }
                [_, Block::Solid(_), Block::Redstone(_), b] if b.is_transparent() => {
                    in_nbs.push((side_up, f))
                }
                _ => {}
            }
        }

        in_nbs
    }
}

impl BlockTrait for Redstone {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        // find biggest signal strength around this block
        let s_new = self
            .in_nbs(p, world)
            .into_iter()
            .map(|(n, f)| {
                let n_block = &world[n];

                n_block.weak_power_dir(f).saturating_sub(1)
            })
            .max()
            .unwrap_or(0);

        // if signal strength has changed, update neighbours
        if self.signal != s_new {
            self.signal = s_new;
            (self.out_nbs(p, world), false)
        } else {
            (vec![], false)
        }
    }
}
