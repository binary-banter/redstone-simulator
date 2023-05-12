use crate::blocks::{Block, Edge};
use crate::world::World;
use itertools::Itertools;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use petgraph::{Incoming, Outgoing};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use petgraph::visit::IntoEdgeReferences;

impl World {
    pub fn prune_graph(&mut self) {
        self.prune_redstone();
        self.prune_dead_nodes();
        self.prune_duplicate_edges();
    }

    fn prune_dead_nodes(&mut self) {
        loop {
            let nodes = self.blocks.node_count();
            self.blocks.retain_nodes(|blocks, y| {
                (blocks.neighbors_directed(y, Outgoing).count() > 0
                    && blocks.neighbors_directed(y, Incoming).count() > 0)
                    || self.probes.contains_left(&y)
                    || self.triggers.contains(&y)
                    || matches!(
                        blocks[y],
                        Block::RedstoneBlock | Block::Torch(_) | Block::Comparator(_)
                    )
            });
            if nodes == self.blocks.node_count() {
                break;
            }
        }
    }

    fn prune_redstone(&mut self) {
        for node in self.blocks.node_indices().collect_vec() {
            if matches!(self.blocks[node], Block::Redstone(_)) && !self.triggers.contains(&node) {
                continue;
            }

            let mut state = vec![(node, Edge::Rear(0))];
            let mut visited: HashSet<(NodeIndex, bool)> = HashSet::new();
            let mut ends = vec![];

            loop {
                let mut new_state = vec![];
                for (s, c) in state {
                    for nb_edge in self.blocks.edges_directed(s, Outgoing) {
                        let nb = nb_edge.target();
                        if visited.contains(&(nb, nb_edge.weight().is_side())) {
                            continue;
                        }
                        visited.insert((nb, nb_edge.weight().is_side()));
                        if self.probes.contains_left(&nb) {
                            ends.push((nb, c + nb_edge.weight()));
                        }
                        if !matches!(self.blocks[nb], Block::Redstone(_)) {
                            ends.push((nb, c + nb_edge.weight()));
                            continue;
                        }
                        new_state.push((nb, c + nb_edge.weight()));
                    }
                }
                if new_state.is_empty() {
                    break;
                }
                state = new_state;
            }

            for (end, i) in ends {
                self.blocks.add_edge(node, end, i);
            }
        }
        self.blocks.retain_nodes(|blocks, n| {
            !matches!(blocks[n], Block::Redstone(_))
                || self.probes.contains_left(&n)
                || self.triggers.contains(&n)
        });
    }

    fn prune_duplicate_edges(&mut self) {
        // (15, 12) (15, 12)
        let mut best_edges: HashMap<(NodeIndex, NodeIndex, bool), EdgeIndex> = HashMap::new();
        let mut edges_to_remove = Vec::new();
        for edge in self.blocks.edge_references() {
            match best_edges.entry((edge.source(), edge.target(), edge.weight().is_side())) {
                Entry::Occupied(mut e) => {
                    if *edge.weight() >= self.blocks[*e.get()] {
                        // remove edge
                        edges_to_remove.push(edge.id());
                    } else{
                        // new best edge
                        edges_to_remove.push(*e.get());
                        e.insert(edge.id());
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(edge.id());
                }
            }
            edge.source();
        }

        edges_to_remove.into_iter().for_each(|e| {
            self.blocks.remove_edge(e);
        })
    }
}
