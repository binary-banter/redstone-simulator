#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl From<&str> for Facing {
    fn from(value: &str) -> Self {
        match value {
            "north" => Facing::North,
            "east" => Facing::East,
            "south" => Facing::South,
            "west" => Facing::West,
            _ => panic!("Not a facing: {value}."),
        }
    }
}

impl Facing {
    pub fn front(self, (x, y, z): (usize, usize, usize)) -> (usize, usize, usize) {
        match self {
            Facing::North => (x, y, z.wrapping_sub(1)),
            Facing::East => (x.wrapping_add(1), y, z),
            Facing::South => (x, y, z.wrapping_add(1)),
            Facing::West => (x.wrapping_sub(1), y, z),
            Facing::Up => (x, y.wrapping_add(1), z),
            Facing::Down => (x, y.wrapping_sub(1), z),
        }
    }

    pub fn rotate_left(self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
            Facing::Up => Facing::Up,
            Facing::Down => Facing::Down,
        }
    }

    pub fn rotate_right(self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
            Facing::Up => Facing::Up,
            Facing::Down => Facing::Down,
        }
    }

    pub fn rev(self) -> Self {
        match self {
            Facing::North => Facing::South,
            Facing::East => Facing::West,
            Facing::South => Facing::North,
            Facing::West => Facing::East,
            Facing::Up => Facing::Down,
            Facing::Down => Facing::Up,
        }
    }
}
