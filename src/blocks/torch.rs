use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, Edge, OutputPower, Updatable};
use crate::world::RedGraph;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::Incoming;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub struct Torch {
    /// Whether the torch is currently lit.
    lit: bool,

    last_update: usize,
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

impl Default for Torch {
    fn default() -> Self {
        Torch {
            lit: true,
            last_update: usize::MAX,
        }
    }
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

impl Torch {
    pub fn with_lit(lit: bool) -> Torch {
        Torch {
            lit,
            last_update: usize::MAX,
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
        self.node = Some(blocks.add_node(Block::Torch(Torch {
            lit: self.lit,
            last_update: usize::MAX,
        })));
    }
}

impl Updatable for Torch {
    #[inline(always)]
    fn update(
        &mut self,
        idx: NodeIndex,
        _tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &RedGraph,
    ) -> bool {
        let s_new = blocks
            .edges_directed(idx, Incoming)
            .any(|edge| match edge.weight() {
                Edge::Rear(s) => blocks[edge.source()].output_power().saturating_sub(*s) > 0,
                Edge::Side(_) => unreachable!(),
            });

        s_new == self.lit
    }

    fn late_updatable(
        &mut self,
        _idx: NodeIndex,
        _updatable: &mut VecDeque<NodeIndex>,
        tick_counter: usize,
    ) -> bool {
        if tick_counter == self.last_update {
            return false;
        }
        self.last_update = tick_counter;

        self.lit = !self.lit;

        true
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
