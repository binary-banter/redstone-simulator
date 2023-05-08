// use crate::world::World;
// use itertools::Itertools;
// use petgraph::graph::NodeIndex;
// use petgraph::prelude::EdgeRef;
// use petgraph::{Incoming, Outgoing};
// use std::collections::HashSet;
//
// impl World {
//     pub fn prune_graph(&mut self) {
//         self.prune_redstone();
//         self.prune_dead_nodes();
//     }
//
//     fn prune_dead_nodes(&mut self) {
//         loop {
//             let nodes = self.blocks.node_count();
//             self.blocks.retain_nodes(|blocks, y| {
//                 (blocks.neighbors_directed(y, Outgoing).count() > 0
//                     && blocks.neighbors_directed(y, Incoming).count() > 0)
//                     || self.probes.contains_left(&y)
//                     || self.triggers.contains(&y)
//                     || matches!(blocks[y], Block::RedstoneBlock | Block::Torch { .. })
//             });
//             if nodes == self.blocks.node_count() {
//                 break;
//             }
//         }
//     }
//
//     fn prune_redstone(&mut self) {
//         for node in self.blocks.node_indices().collect_vec() {
//             if matches!(self.blocks[node], Block::Redstone(_)) && !self.triggers.contains(&node) {
//                 continue;
//             }
//
//             let mut state: Vec<(NodeIndex, u8)> = vec![(node, 0)];
//             let mut visited: HashSet<NodeIndex> = HashSet::new();
//             let mut ends: Vec<(NodeIndex, u8)> = vec![];
//
//             loop {
//                 let mut new_state: Vec<(NodeIndex, u8)> = vec![];
//                 for (s, c) in state {
//                     for nb_edge in self.blocks.edges_directed(s, Outgoing) {
//                         let nb = nb_edge.target();
//                         if visited.contains(&nb) {
//                             continue;
//                         }
//                         visited.insert(nb);
//                         if self.probes.contains_left(&nb) {
//                             ends.push((nb, c + nb_edge.weight()));
//                         }
//                         if !matches!(self.blocks[nb], Block::Redstone(_))
//                             || self
//                                 .blocks
//                                 .neighbors_directed(nb, Outgoing)
//                                 .any(|nb2| matches!(self.blocks[nb2], Block::Comparator { .. }))
//                         {
//                             ends.push((nb, c + nb_edge.weight()));
//                             continue;
//                         }
//                         new_state.push((nb, c + nb_edge.weight()));
//                     }
//                 }
//                 if new_state.is_empty() {
//                     break;
//                 }
//                 state = new_state;
//             }
//
//             for (end, i) in ends {
//                 self.blocks.add_edge(node, end, i);
//             }
//         }
//         self.blocks.retain_nodes(|blocks, n| {
//             !matches!(blocks[n], Block::Redstone(_))
//                 || self.probes.contains_left(&n)
//                 || self.triggers.contains(&n)
//                 || blocks
//                     .neighbors_directed(n, Outgoing)
//                     .any(|nb2| matches!(blocks[nb2], Block::Comparator { .. }))
//         });
//     }
// }
