use std::ops::Add;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Edge {
    Rear(u8),
    Side(u8),
}

impl Edge {
    pub fn is_side(&self) -> bool {
        match self {
            Edge::Rear(_) => false,
            Edge::Side(_) => true,
        }
    }

    pub fn strength_loss(&self) -> u8 {
        match self {
            Edge::Rear(v) => *v,
            Edge::Side(v) => *v,
        }
    }
}

impl Add<&Edge> for Edge {
    type Output = Self;

    fn add(self, rhs: &Edge) -> Self::Output {
        match (self, rhs) {
            (Edge::Rear(s1), Edge::Side(s2)) => Edge::Side(s1 + s2),
            (Edge::Rear(s1), Edge::Rear(s2)) => Edge::Rear(s1 + s2),
            _ => unreachable!(),
        }
    }
}
