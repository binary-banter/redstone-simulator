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
}
