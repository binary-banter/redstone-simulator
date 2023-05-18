use crate::blocks::CBlock;
use crate::world::CBlockGraph;
use petgraph::prelude::{EdgeRef, NodeIndex};
use petgraph::visit::{IntoEdgeReferences, IntoNodeReferences};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_dot(
    cblocks: &CBlockGraph,
    pos: &HashMap<NodeIndex, (isize, isize, isize)>,
    path: &Path,
) {
    let mut w = File::create(path).unwrap();
    writeln!(w, "digraph world {{").unwrap();
    for (node, cblock) in cblocks.node_references() {
        let (label, color) = match cblock {
            CBlock::Redstone(_) => unreachable!(),
            CBlock::SolidWeak(_) => unreachable!(),
            CBlock::SolidStrong(_) => unreachable!(),
            CBlock::Trigger(_) => ("trigger", "#FFE119"),
            CBlock::Probe(_) => ("probe", "#42D4F4"),
            CBlock::Repeater(_) => ("repeater", "#469990"),
            CBlock::SRepeater(_) => ("srepeater", "#3CB44B"),
            CBlock::RedstoneBlock(_) => ("redstone_block", "#F58231"),
            CBlock::Torch(_) => ("torch", "#E6194B"),
            CBlock::Comparator(_) => ("comparator", "#911EB4"),
        };
        if let Some(pos) = pos.get(&node) {
            writeln!(
                w,
                "{}[label=\"{label}\",color={color}, world_pos=\"{:?}\"];",
                node.index(),
                pos
            )
            .unwrap();
        } else {
            writeln!(w, "{}[label=\"{label}\",color={color}];", node.index()).unwrap();
        }
    }
    for edge in cblocks.edge_references() {
        let color = if edge.weight().is_side() {
            "#A9A9A9"
        } else {
            "#000000"
        };
        writeln!(
            w,
            "{} -> {}[label=\"{}\",color={color}];",
            edge.source().index(),
            edge.target().index(),
            edge.weight().strength_loss()
        )
        .unwrap();
    }

    writeln!(w, "}}").unwrap();
}
