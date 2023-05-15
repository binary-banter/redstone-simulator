use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, BlockConnections, InputSide, ToBlock};
use crate::world::data::{neighbours, TileMap};
use nbt::Value;

#[derive(Clone, Debug, Default)]
pub struct CProbe {
    /// Name of the probe. Uses the first line of any neighbouring sign it finds.
    pub name: String,
}

impl BlockConnections for CProbe {
    fn can_output(&self, _facing: Facing) -> bool {
        false
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        Some(InputSide::Rear)
    }
}
impl ToBlock for CProbe {
    fn to_block(&self) -> Block {
        Block::Redstone(Redstone::default())
    }
}

impl CProbe {
    pub fn update_from_tile(&mut self, p: (usize, usize, usize), tile_map: &TileMap) {
        self.name = neighbours(p)
            .find_map(|p| {
                tile_map.get(&p).and_then(|b| {
                    if b.id == "minecraft:sign" {
                        if let Some(Value::String(s)) = b.props.get("Text1") {
                            let j = serde_json::from_str::<serde_json::Value>(&s).unwrap();
                            return Some(
                                j.as_object()
                                    .unwrap()
                                    .get("text")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                            );
                        }
                    }
                    None
                })
            })
            .unwrap_or(format!("{},{},{}", p.0, p.1, p.2))
    }
}
