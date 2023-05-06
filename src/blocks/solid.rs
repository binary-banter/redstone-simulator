use crate::blocks::facing::Facing;
use crate::blocks::solid::SolidPower::{Strong, Weak};
use crate::blocks::{Block, BlockTrait};
use crate::world_data::WorldData;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SolidPower {
    Weak(u8),
    Strong(u8),
}

impl From<SolidPower> for u8 {
    fn from(v: SolidPower) -> Self {
        match v {
            Weak(v) => v,
            Strong(v) => v,
        }
    }
}

impl Eq for SolidPower {}

impl PartialOrd<Self> for SolidPower {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SolidPower {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Weak(0), Strong(0)) => Ordering::Equal,
            (Strong(0), Weak(0)) => Ordering::Equal,
            (Weak(1..), Strong(0)) => Ordering::Greater,
            (Strong(0), Weak(1..)) => Ordering::Less,
            (Weak(_), Strong(1..)) => Ordering::Less,
            (Strong(1..), Weak(_)) => Ordering::Greater,
            (Weak(v1), Weak(v2)) => v1.cmp(v2),
            (Strong(v1), Strong(v2)) => v1.cmp(v2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Solid {
    /// Can be `Weak` or `Strong` powered.
    pub signal: SolidPower,
}

impl Solid {
    fn out_nbs(
        &self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> impl Iterator<Item = (usize, usize, usize)> {
        world.neighbours(p)
    }

    fn input_signal(&self, b: &Block, f: Facing) -> SolidPower {
        match b {
            Block::Solid(_) => Weak(0),
            Block::Redstone(v) => Weak(v.output_signal(f)),
            Block::RedstoneBlock => Weak(0),
            Block::Trigger(_) => Weak(0),
            Block::Repeater(v) => Strong(v.output_signal(f)),
            Block::Comparator(v) => Strong(v.output_signal(f)),
            Block::Torch(v) => Strong(if f == Facing::Down {
                v.output_signal(f)
            } else {
                0
            }),
            Block::Air => Weak(0),
        }
    }

    pub fn output_signal(&self) -> SolidPower {
        self.signal
    }
}

impl BlockTrait for Solid {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
        updates: &mut Vec<(usize, usize, usize)>,
    ) -> bool {
        let s_new = world
            .neighbours_and_facings(p)
            .into_iter()
            .map(|(n, f)| self.input_signal(&world[n], f))
            .max()
            .unwrap_or(Weak(0));

        // if signal strength has changed, update neighbours
        if self.signal != s_new {
            self.signal = s_new;
            updates.extend(self.out_nbs(p, world));
        }
        false
    }
}
