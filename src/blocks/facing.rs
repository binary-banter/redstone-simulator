#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl Facing {
    pub fn reverse(self) -> Self {
        match self {
            Facing::North => Facing::South,
            Facing::East => Facing::West,
            Facing::South => Facing::North,
            Facing::West => Facing::East,
            Facing::Up => Facing::Down,
            Facing::Down => Facing::Up,
        }
    }

    pub fn front(self, (x, y, z): (usize, usize, usize)) -> (usize, usize, usize) {
        match self {
            Facing::North => (x, y, z - 1),
            Facing::East => (x + 1, y, z),
            Facing::South => (x, y, z + 1),
            Facing::West => (x - 1, y, z),
            Facing::Up => (x, y + 1, z),
            Facing::Down => (x, y - 1, z),
        }
    }

    pub fn back(self, p: (usize, usize, usize)) -> (usize, usize, usize) {
        self.reverse().front(p)
    }
}
