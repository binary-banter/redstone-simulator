use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, Edge, OutputPower, Updatable};
use crate::world::RedGraph;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::{Incoming, Outgoing};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Torch {
    /// Whether the torch is currently lit.
    lit: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct CTorch {
    /// Whether the torch is currently lit.
    lit: bool,

    /// Direction the torch points in.
    facing: Facing,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,
}

impl OutputPower for Torch {
    fn output_power(&self) -> u8 {
        if self.lit {
            15
        } else {
            0
        }
    }
}

impl BlockConnections for CTorch {
    fn can_output(&self, _facing: Facing) -> Option<NodeIndex> {
        self.node
    }

    fn can_input(&self, facing: Facing) -> (Option<NodeIndex>, bool) {
        if self.facing == facing {
            (self.node, false)
        } else {
            (None, false)
        }
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, _add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        self.node = Some(blocks.add_node(Block::Torch(Torch { lit: self.lit })));
    }
}

impl Updatable for Torch {
    fn update(
        &mut self,
        idx: NodeIndex,
        _tick_updatable: &mut Vec<NodeIndex>,
        blocks: &mut RedGraph,
    ) -> bool {
        let s_new = blocks
            .edges_directed(idx, Incoming)
            .any(|edge| match edge.weight() {
                Edge::Rear(s) => blocks[edge.source()].output_power().saturating_sub(*s) > 0,
                Edge::Side(_) => false,
            });

        s_new == self.lit
    }

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut Vec<NodeIndex>,
        blocks: &mut RedGraph,
    ) {
        self.lit = !self.lit;

        updatable.extend(blocks.neighbors_directed(idx, Outgoing));
    }
}

impl From<HashMap<&str, &str>> for CTorch {
    fn from(meta: HashMap<&str, &str>) -> Self {
        let lit = meta.get("lit").map(|&x| x == "true").unwrap();

        let facing = meta
            .get("facing")
            .map(|&f| Facing::from(f))
            .unwrap_or(Facing::Up);

        CTorch {
            lit,
            facing,
            node: None,
        }
    }
}
