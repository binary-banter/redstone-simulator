use crate::blocks::facing::Facing;
use crate::blocks::CBlock;
use std::ops::{Index, IndexMut};

pub struct WorldData(pub Vec<Vec<Vec<CBlock>>>);

impl WorldData {
    /// Returns the coordinates of neighbouring blocks relative to the given position.
    pub fn neighbours(
        &self,
        (x, y, z): (usize, usize, usize),
    ) -> impl Iterator<Item = (usize, usize, usize)> {
        [
            (x.wrapping_sub(1), y, z),
            (x.wrapping_add(1), y, z),
            (x, y.wrapping_sub(1), z),
            (x, y.wrapping_add(1), z),
            (x, y, z.wrapping_sub(1)),
            (x, y, z.wrapping_add(1)),
        ]
        .into_iter()
    }

    /// Returns the coordinates and facing of neighbouring blocks relative to the given position.
    pub fn neighbours_and_facings(
        &self,
        (x, y, z): (usize, usize, usize),
    ) -> impl Iterator<Item = ((usize, usize, usize), Facing)> {
        [
            ((x.wrapping_sub(1), y, z), Facing::West),
            ((x.wrapping_add(1), y, z), Facing::East),
            ((x, y.wrapping_sub(1), z), Facing::Down),
            ((x, y.wrapping_add(1), z), Facing::Up),
            ((x, y, z.wrapping_sub(1)), Facing::North),
            ((x, y, z.wrapping_add(1)), Facing::South),
        ]
        .into_iter()
    }
}

impl Index<(usize, usize, usize)> for WorldData {
    type Output = CBlock;

    fn index(&self, (x, y, z): (usize, usize, usize)) -> &Self::Output {
        self.0
            .get(x)
            .and_then(|l| l.get(y).and_then(|l| l.get(z)))
            .unwrap_or(&CBlock::Air)
    }
}

impl IndexMut<(usize, usize, usize)> for WorldData {
    fn index_mut(&mut self, (x, y, z): (usize, usize, usize)) -> &mut Self::Output {
        &mut self.0[x][y][z]
    }
}
