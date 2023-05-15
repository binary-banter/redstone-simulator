use crate::blocks::srepeater_powered;
use crate::blocks::srepeater_unpowered;
use crate::blocks::{Block, Edge, OutputPower};
use crate::world::World;
use itertools::Itertools;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use petgraph::{Incoming, Outgoing};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

impl World {
    pub fn prune_graph(&mut self) {
        self.prune_redstone();
        self.prune_duplicate_edges();
        self.prune_too_long_edges();
        self.prune_groups();
        self.prune_duplicate_edges();
        self.prune_irrelevant();
        self.replace_simple_repeaters();
        self.prune_dead_nodes();
    }

    fn prune_dead_nodes(&mut self) {
        loop {
            let nodes = self.blocks.node_count();
            self.blocks.retain_nodes(|blocks, y| {
                // block has both input and output
                if blocks.neighbors_directed(y, Outgoing).count() > 0
                    && blocks.neighbors_directed(y, Incoming).count() > 0
                {
                    return true;
                }

                match &blocks[y] {
                    // retain triggers and probes
                    Block::Redstone(_) => {
                        self.probes.contains_left(&y) || self.triggers.contains(&y)
                    }
                    Block::Repeater(_) => false,
                    Block::SRepeater(_) => false,
                    // retain torches with outputs (can be used as redstone blocks)
                    Block::Torch(_) => blocks.neighbors_directed(y, Outgoing).count() > 0,
                    Block::Comparator(c) => {
                        c.contains_entity_power()
                            && blocks.neighbors_directed(y, Outgoing).count() > 0
                    }
                }
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
        let mut best_edges: HashMap<(NodeIndex, NodeIndex, bool), EdgeIndex> = HashMap::new();
        let mut edges_to_remove = Vec::new();
        for edge in self.blocks.edge_references() {
            match best_edges.entry((edge.source(), edge.target(), edge.weight().is_side())) {
                Entry::Occupied(mut e) => {
                    if *edge.weight() >= self.blocks[*e.get()] {
                        // remove edge
                        edges_to_remove.push(edge.id());
                    } else {
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

    fn prune_too_long_edges(&mut self) {
        self.blocks.retain_edges(|g, e| g[e].strength_loss() < 15)
    }

    fn prune_groups(&mut self) {
        let mut todo = self
            .blocks
            .node_indices()
            .filter(|i| matches!(self.blocks[*i], Block::Repeater(_) | Block::Torch(_)))
            .collect_vec();

        while let Some(idx) = todo.pop() {
            let mut repeaters: HashMap<usize, Vec<NodeIndex>> = HashMap::new();
            let mut torches = Vec::new();

            for n_idx in self.blocks.neighbors_directed(idx, Outgoing) {
                // Only group items with a single parent
                if self.blocks.neighbors_directed(n_idx, Incoming).count() > 1 {
                    continue;
                }

                // Only group repeaters and torches.
                match &self.blocks[n_idx] {
                    Block::Repeater(v) => {
                        repeaters.entry(v.delay() as usize).or_default().push(n_idx);
                    }
                    Block::Torch(_) => torches.push(n_idx),
                    _ => continue,
                }
            }

            if torches.len() > 1 {
                todo.push(self.merge_nodes(torches.into_iter()));
            }
            for (_, repeaters) in repeaters.into_iter() {
                if repeaters.len() > 1 {
                    todo.push(self.merge_nodes(repeaters.into_iter()));
                }
            }
        }
    }

    fn merge_nodes(&mut self, mut nodes: impl Iterator<Item = NodeIndex>) -> NodeIndex {
        let first = nodes.next().unwrap();
        for other in nodes {
            let edges = self
                .blocks
                .edges_directed(other, Outgoing)
                .map(|e| e.id())
                .collect_vec();
            for edge in edges {
                self.blocks.add_edge(
                    first,
                    self.blocks.edge_endpoints(edge).unwrap().1,
                    self.blocks[edge],
                );
            }
            self.blocks.remove_node(other);
        }
        first
    }

    fn prune_irrelevant(&mut self) {
        let mut visited: HashSet<NodeIndex> = HashSet::from_iter(
            self.probes
                .left_values()
                .cloned()
                .chain(self.triggers.iter().cloned()),
        );
        let mut todo: Vec<NodeIndex> = self.probes.left_values().cloned().collect();

        while let Some(idx) = todo.pop() {
            for nb in self.blocks.neighbors_directed(idx, Incoming) {
                if visited.contains(&nb) {
                    continue;
                }
                visited.insert(nb);
                todo.push(nb);
            }
        }

        self.blocks.retain_nodes(|_, n| visited.contains(&n));
    }

    fn replace_simple_repeaters(&mut self) {
        self.blocks
            .node_indices()
            .collect_vec()
            .into_iter()
            .for_each(|idx| {
                let Some(Block::Repeater(r)) = self.blocks.node_weight(idx) else{
                    return
                };

                if r.delay() != 1 {
                    return;
                }

                if self
                    .blocks
                    .edges_directed(idx, Outgoing)
                    .any(|edge| matches!(edge.weight(), Edge::Side(_)))
                {
                    return;
                }

                if self
                    .blocks
                    .edges_directed(idx, Incoming)
                    .any(|edge| matches!(edge.weight(), Edge::Side(_)))
                {
                    return;
                }

                *self.blocks.node_weight_mut(idx).unwrap() = if r.output_power() > 0 {
                    srepeater_powered()
                } else {
                    srepeater_unpowered()
                };
            });
    }
}
