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

        if x != 0 {
            vec.push(((x - 1, y, z), Facing::West)).unwrap();
        }
        if x != self.0.len() - 1 {
            vec.push(((x + 1, y, z), Facing::East)).unwrap();
        }
        if y != 0 {
            vec.push(((x, y - 1, z), Facing::Up)).unwrap();
        }
        if y != self.0[0].len() - 1 {
            match self[(x, y + 1, z)] {
                Block::Redstone(_) => {}
                _ => vec.push(((x, y + 1, z), Facing::Down)).unwrap(),
            }
        }
        if z != 0 {
            vec.push(((x, y, z - 1), Facing::North)).unwrap();
        }
        if z != self.0[0][0].len() - 1 {
            vec.push(((x, y, z + 1), Facing::South)).unwrap();
        }

        vec.into_iter()
    }
}

impl Index<(usize, usize, usize)> for WorldData {
    type Output = Block;

    fn index(&self, (x, y, z): (usize, usize, usize)) -> &Self::Output {
        &self.0[x][y][z]
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
