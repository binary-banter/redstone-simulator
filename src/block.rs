use petgraph::graph::NodeIndex;

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

#[derive(Clone, Debug)]
pub enum Block {
    Redstone(u8),
    Repeater {
        /// Whether the repeater is currently powered
        powered: bool,
        /// Next power when count reaches the repeater delay
        next_powered: bool,
        /// The repeater delay
        delay: u8,
        /// Number of ticks passed since a new input signal was detected.
        count: u8,
    },
    RedstoneBlock,
    Torch {
        /// Whether the torch is currently lit
        lit: bool,
    },
    Comparator {
        signal: u8,
        next_signal: u8,
        mode: ComparatorMode,
        rear: NodeIndex,
        side: NodeIndex,
    },
}

impl Block {
    pub fn output_power(&self) -> u8 {
        match *self {
            Block::Redstone(v) => v,
            Block::RedstoneBlock => 15,
            Block::Repeater { powered: true, .. } => 15,
            Block::Repeater { powered: false, .. } => 0,
            Block::Comparator { signal, .. } => signal,
            Block::Torch { lit: true } => 15,
            Block::Torch { lit: false } => 0,
        }
    }
}
