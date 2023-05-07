#[derive(Clone, Debug)]
pub enum Block {
    Redstone(u8),
    Solid(u8),
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
    }
}

impl Block {
    pub fn output_power(&self) -> u8 {
        match *self {
            Block::Solid(v) => v,
            Block::Redstone(v) => v,
            Block::RedstoneBlock => 15,
            Block::Repeater { powered: true, .. } => 15,
            Block::Repeater { powered: false, .. } => 0,
            // Block::Comparator(v) => v.output_signal(f),
            Block::Torch { lit: true} => 15,
            Block::Torch { lit: false} => 0,
        }
    }
}
