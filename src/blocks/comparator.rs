use std::cmp::max;
use crate::blocks::{BlockTrait, BlockTraitLate};
use crate::blocks::facing::Facing;
use crate::world_data::WorldData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ComparatorMode {
    Compare,
    Subtract
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

impl BlockTrait for Comparator {
    fn update(&mut self, pos: (usize, usize, usize), world: &WorldData) -> (Vec<(usize, usize, usize)>, bool) {
        let rear = world[self.facing.front(pos)].weak_power_dir(self.facing) + 1;
        let left = world[self.facing.rotate_right().front(pos)].weak_power_dir(self.facing.rotate_right());
        let right = world[self.facing.rotate_left().front(pos)].weak_power_dir(self.facing.rotate_left());

        self.next_signal = match self.mode {
            ComparatorMode::Compare => if left <= rear && right <= rear { rear } else { 0 },
            ComparatorMode::Subtract => rear.saturating_sub(max(left, right)),
        };

        (vec![], self.signal != self.next_signal)
    }
}

impl BlockTraitLate for Comparator {
    fn update_late(&mut self, pos: (usize, usize, usize), world: &WorldData) -> Vec<(usize, usize, usize)> {
        self.signal = self.next_signal;
        world.neighbours(pos)
    }
}