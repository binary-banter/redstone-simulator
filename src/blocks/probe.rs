use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, BlockConnections};
use crate::world::data::{neighbours, TileMap};
use crate::world::BlockGraph;
use nbt::Value;
use petgraph::stable_graph::NodeIndex;

#[derive(Clone, Debug, Default)]
pub struct CProbe {
    /// Name of the probe. Uses the first line of any neighbouring sign it finds.
    pub name: String,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    pub node: Option<NodeIndex>,
}

impl BlockConnections for CProbe {
    fn can_output(&self, _facing: Facing) -> Option<NodeIndex> {
        None
    }

    fn can_input(&self, _facing: Facing) -> (Option<NodeIndex>, bool) {
        (self.node, false)
    }

    fn add_node(&mut self, blocks: &mut BlockGraph) {
        let idx = blocks.add_node(Block::Redstone(Redstone::default()));
        self.node = Some(idx);
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
