use crate::blocks::facing::Facing;
use crate::blocks::repeater::Repeater;
use crate::blocks::solid::Solid;
use crate::blocks::trigger::Trigger;
use crate::blocks::{Block, BlockTrait};
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConnectionDirection {
    None,
    Up,
    Side,
}

impl ConnectionDirection {
    pub fn from_str(s: &str) -> ConnectionDirection {
        // `down` is not supported by minecraft so it deliberately not an option.
        match s {
            "none" => ConnectionDirection::None,
            "side" => ConnectionDirection::Side,
            "up" => ConnectionDirection::Up,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionDirections {
    pub north: ConnectionDirection,
    pub east: ConnectionDirection,
    pub south: ConnectionDirection,
    pub west: ConnectionDirection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Redstone {
    /// Ranges from 0 to 15 inclusive.
    pub signal: u8,

    /// North East South West
    pub out_dirs: ConnectionDirections,
}

impl BlockTrait for Redstone {
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
    ) -> Vec<(usize, usize, usize)> {
        let mut in_nbs = vec![
            (x.wrapping_sub(1), y, z),
            (x.wrapping_add(1), y, z),
            (x, y.wrapping_sub(1), z),
            (x, y.wrapping_add(1), z),
            (x, y, z.wrapping_sub(1)),
            (x, y, z.wrapping_add(1)),
        ];

        let top = (x,y.wrapping_add(1),z);
        for f in [Facing::North, Facing::East, Facing::South, Facing::West] {
            let side = f.front((x, y, z));
            let side_down = (side.0, side.1.wrapping_sub(1), side.2);
            let side_up = (side.0, side.1.wrapping_add(1), side.2);

            match [side_down, side, side_up, top].map(|n| &world[n]) {
                [Block::Redstone(_), b, _, _] if b.is_transparent() => {
                    in_nbs.push(side_down);
                }
                [_, Block::Solid(_), Block::Redstone(_), b] if b.is_transparent() => {
                    in_nbs.push(side_up)
                }
                _ => {}
            }
        }

        in_nbs
    }

    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {

        // find biggest signal strength around this block
        let s_new = self.in_nbs(p,world)
            .into_iter()
            .map(|n| {
                let n_block = &world[n];
                match n_block {
                    Block::Redstone(Redstone { signal: ns, .. }) => ns.saturating_sub(1),
                    Block::Repeater(Repeater {
                        signal: 16,
                        facing: nf,
                        ..
                    }) if nf.back(n) == p => 15,
                    Block::Trigger(Trigger { signal: 16 }) | Block::Solid(Solid { signal: 16 }) => {
                        15
                    }
                    Block::Air | Block::Repeater(_) | Block::Solid(_) | Block::Trigger(_) => 0,
                    Block::Torch(_) => todo!(),
                }
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
