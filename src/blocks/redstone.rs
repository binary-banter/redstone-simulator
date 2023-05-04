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
    pub in_dirs: Vec<(usize, usize, usize)>,
    pub out_dirs: ConnectionDirections,
}

impl BlockTrait for Redstone {
    fn out_nbs(
        &self,
        (x, y, z): (usize, usize, usize),
        _world: &WorldData,
    ) -> Vec<(usize, usize, usize)> {
        vec![
            (x.wrapping_sub(1), y, z),
            (x.wrapping_add(1), y, z),
            (x, y.wrapping_sub(1), z),
            (x, y, z.wrapping_sub(1)),
            (x, y, z.wrapping_add(1)),
        ]
    }

    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        let mut in_nbs = world.neighbours(p);
        in_nbs.extend(&self.in_dirs);

        // find biggest signal strength around this block
        let s_new = in_nbs
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
        let marked_neighbours = if self.signal != s_new {
            self.signal = s_new;
            self.out_nbs(p, world)
        } else {
            vec![]
        };

        (marked_neighbours, false)
    }
}
