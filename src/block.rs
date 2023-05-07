#[derive(Clone, Debug)]
pub enum Block {
    Redstone(u8),
    Solid(u8),
}

impl Block {
    pub fn output_power(&self) -> u8 {
        match *self {
            Block::Solid(v) => v,
            Block::Redstone(v) => v,
            // Block::RedstoneBlock => 15,
            // Block::Trigger(v) => v.output_signal(),
            // Block::Repeater(v) => v.output_signal(f),
            // Block::Comparator(v) => v.output_signal(f),
            // Block::Torch(v) => v.output_signal(f),
            // Block::Air => 0,
        }
    }
}
