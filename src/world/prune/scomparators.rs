use std::collections::HashMap;
use std::ops::SubAssign;
use itertools::Itertools;
use petgraph::{Incoming, Outgoing};
use petgraph::prelude::{EdgeRef, NodeIndex};
use crate::blocks::{CBlock, Edge};
use crate::blocks::scomparator::CSComparator;
use crate::world::CBlockGraph;

pub fn replace_simple_comparators(cblocks: &mut CBlockGraph){
    let mut blocks = HashMap::new();

    for idx in cblocks.node_indices().collect_vec().clone(){
        if !matches!(cblocks[idx], CBlock::Comparator(_)){
            continue;
        }

        for rear in 1..=15u8 {
            for side in 0..=15u8 {
                // Todo make scomparator/repeater
                // output_if_rear_and_side_on = rear.saturating_sub(side);
                blocks.insert((idx, rear, side), cblocks.add_node(CBlock::SComparator(CSComparator{ rear, side })));
            }
        }
    }

    for (&(c_idx, rear, side), &s_idx) in &blocks {
        let mut rear_inputs: Vec<NodeIndex> = Vec::new();
        for edge in cblocks.edges_directed(c_idx, Incoming).filter(|e| !e.weight().is_side()) {
            match cblocks[edge.source()] {
                CBlock::Comparator(_) => {},
                _ => {
                    if 15 - edge.weight().strength_loss() == rear {
                        rear_inputs.push(edge.source());
                    }
                }
            }
        }

        let mut side_inputs: Vec<NodeIndex> = Vec::new();
        for edge in cblocks.edges_directed(c_idx, Incoming).filter(|e| e.weight().is_side()) {
            match cblocks[edge.source()] {
                CBlock::Comparator(_) => {},
                _ => {
                    if 15 - edge.weight().strength_loss() == side {
                        side_inputs.push(edge.source());
                    }
                }
            }
        }

        let output_power = rear.saturating_sub(side);
        let mut rear_outputs: Vec<NodeIndex> = Vec::new();
        let mut side_outputs: Vec<NodeIndex> = Vec::new();
        for edge in cblocks.edges_directed(c_idx, Outgoing) {
            match cblocks[edge.target()] {
                CBlock::Comparator(_) => {
                    for (&(x_old, x_rear, x_side), &x) in &blocks{
                        if edge.target() == x_old {
                            if !edge.weight().is_side() && output_power == x_rear {
                                rear_outputs.push(x);
                            }
                            if edge.weight().is_side() && output_power == x_side {
                                side_outputs.push(x);
                            }
                        }
                    }
                },
                _ => {
                    if output_power >= 15 - edge.weight().strength_loss() {
                        if edge.weight().is_side() {
                            side_outputs.push(edge.target());
                        } else {
                            rear_outputs.push(edge.target());
                        }
                    }
                }
            }
        }

        for n in rear_inputs {
            cblocks.add_edge(n, s_idx, Edge::Rear(0));
        }
        for n in side_inputs {
            cblocks.add_edge(n, s_idx, Edge::Side(0));
        }
        for n in rear_outputs {
            cblocks.add_edge(s_idx, n, Edge::Rear(0));
        }
        for n in side_outputs {
            cblocks.add_edge(s_idx, n, Edge::Side(0));
        }
    }

    cblocks.retain_nodes(|g, n| !matches!(g[n], CBlock::Comparator(_)));
}