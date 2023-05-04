use crate::blocks::facing::Facing;
use crate::blocks::Block;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

pub struct WorldData(pub Vec<Vec<Vec<Block>>>);

impl WorldData {
    /// Returns the coordinates and facing of neighbouring blocks relative to the given position.
    pub fn neighbours(
        &self,
        (x, y, z): (usize, usize, usize),
    ) -> impl Iterator<Item = ((usize, usize, usize), Facing)> {
        let mut vec: heapless::Vec<_, 6> = heapless::Vec::new();

        vec.push(((x.wrapping_sub(1), y, z), Facing::West)).unwrap();
        vec.push(((x.wrapping_add(1), y, z), Facing::East)).unwrap();
        vec.push(((x, y.wrapping_sub(1), z), Facing::Up)).unwrap();
        vec.push(((x, y.wrapping_add(1), z), Facing::Down)).unwrap();
        vec.push(((x, y, z.wrapping_sub(1)), Facing::North)).unwrap();
        vec.push(((x, y, z.wrapping_add(1)), Facing::South)).unwrap();

        vec.into_iter()
    }
}

impl Index<(usize, usize, usize)> for WorldData {
    type Output = Block;

    fn index(&self, (x, y, z): (usize, usize, usize)) -> &Self::Output {
        self.0.get(x).map(|l| l.get(y).map(|l| l.get(z))).flatten().flatten().unwrap_or(&Block::Air)
    }
}

impl IndexMut<(usize, usize, usize)> for WorldData {
    fn index_mut(&mut self, (x, y, z): (usize, usize, usize)) -> &mut Self::Output {
        &mut self.0[x][y][z]
    }
}

impl Display for WorldData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.0[0].len()).rev() {
            for z in 0..self.0[0][0].len() {
                for x in 0..self.0.len() {
                    write!(f, "{}", self.0[x][y][z])?;
                }
                writeln!(f)?;
            }
            writeln!(f, "-----------------------")?;
        }
        Ok(())
    }
}
