use crate::blocks::facing::Facing;
use crate::blocks::Block;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

pub struct WorldData(pub Vec<Vec<Vec<Block>>>);

impl WorldData {
    /// Returns the coordinates of neighbouring blocks relative to the given position.
    pub fn neighbours(&self, p: (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
        self.neighbours_and_facings(p)
            .into_iter()
            .map(|(n, _)| n)
            .collect()
    }

    /// Returns the coordinates and facing of neighbouring blocks relative to the given position.
    pub fn neighbours_and_facings(
        &self,
        (x, y, z): (usize, usize, usize),
    ) -> Vec<((usize, usize, usize), Facing)> {
        let mut vec = Vec::new();

        vec.push(((x.wrapping_sub(1), y, z), Facing::West));
        vec.push(((x.wrapping_add(1), y, z), Facing::East));
        vec.push(((x, y.wrapping_sub(1), z), Facing::Down));
        vec.push(((x, y.wrapping_add(1), z), Facing::Up));
        vec.push(((x, y, z.wrapping_sub(1)), Facing::North));
        vec.push(((x, y, z.wrapping_add(1)), Facing::South));

        vec
    }
}

impl Index<(usize, usize, usize)> for WorldData {
    type Output = Block;

    fn index(&self, (x, y, z): (usize, usize, usize)) -> &Self::Output {
        self.0
            .get(x)
            .and_then(|l| l.get(y).and_then(|l| l.get(z)))
            .unwrap_or(&Block::Air)
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
