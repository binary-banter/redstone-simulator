use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockTrait, BlockTraitLate};
use crate::world_data::WorldData;
use std::cmp::max;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ComparatorMode {
    Compare,
    Subtract,
}

impl From<&str> for ComparatorMode {
    fn from(s: &str) -> Self {
        match s {
            "compare" => Self::Compare,
            "subtract" => Self::Subtract,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comparator {
    pub signal: u8,
    pub next_signal: u8,

    /// Direction the input side of the repeater faces.
    pub facing: Facing,

    pub mode: ComparatorMode,
}

impl Comparator {
    fn input_signal_back(&self, b: &Block, f: Facing) -> u8 {
        b.output_power(f)
    }

    fn input_signal_side(&self, b: &Block, f: Facing) -> u8 {
        match b {
            Block::Solid(_) => 0,
            Block::Redstone(v) => v.output_signal(f),
            Block::RedstoneBlock => 15,
            Block::Trigger(_) => 0,
            Block::Repeater(v) => v.output_signal(f),
            Block::Comparator(v) => v.output_signal(f),
            Block::Torch(_) => 0,
            Block::Air => 0,
        }
    }

    pub fn output_signal(&self, f: Facing) -> u8 {
        if f == self.facing {
            self.signal
        } else {
            0
        }
    }
}

impl BlockTrait for Comparator {
    fn update(
        &mut self,
        p: (usize, usize, usize),
        world: &WorldData,
    ) -> (Vec<(usize, usize, usize)>, bool) {
        let rear = self.input_signal_back(&world[self.facing.front(p)], self.facing);
        let left = self.input_signal_side(
            &world[self.facing.rotate_right().front(p)],
            self.facing.rotate_right(),
        );
        let right = self.input_signal_side(
            &world[self.facing.rotate_left().front(p)],
            self.facing.rotate_left(),
        );

        self.next_signal = match self.mode {
            ComparatorMode::Compare if left <= rear && right <= rear => rear,
            ComparatorMode::Compare => 0,
            ComparatorMode::Subtract => rear.saturating_sub(max(left, right)),
        };

        (vec![], self.signal != self.next_signal)
    }
}

impl BlockTraitLate for Comparator {
    fn update_late(
        &mut self,
        pos: (usize, usize, usize),
        world: &WorldData,
    ) -> Vec<(usize, usize, usize)> {
        self.signal = self.next_signal;
        world.neighbours(pos)
    }
}
