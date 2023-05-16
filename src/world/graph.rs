use crate::blocks::{Block, CBlock, ToBlock};
use crate::world::CBlockGraph;
use bumpalo::Bump;
use itertools::Itertools;
use petgraph::prelude::{EdgeRef, NodeIndex};
use petgraph::visit::IntoNodeReferences;
use petgraph::{Incoming, Outgoing};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem;

pub struct GNode<N: 'static, E: 'static> {
    pub weight: N,
    pub outgoing: &'static [GEdge<N, E>],
    pub incoming_rear: &'static [GEdge<N, E>],
    pub incoming_side: &'static [GEdge<N, E>],
}

pub struct GEdge<N: 'static, E: 'static> {
    pub weight: E,
    pub node: &'static GNode<N, E>,
}

pub struct FastGraph<N: 'static, E: 'static> {
    phantom: PhantomData<(N, E)>,
}

impl FastGraph<Block, u8> {
    pub fn from_petgraph(
        g: &CBlockGraph,
        mut callback: impl FnMut(&CBlock, &'static GNode<Block, u8>),
    ) -> Self {
        let bump: &'static Bump = Box::leak(Box::new(Bump::new()));

        let mut nodes: HashMap<NodeIndex, &'static mut GNode<Block, u8>> = HashMap::new();
        for (idx, block) in g.node_references() {
            let block_ref = bump.alloc(GNode {
                weight: block.to_block(),
                outgoing: &[],
                incoming_rear: &[],
                incoming_side: &[],
            });
            nodes.insert(idx, block_ref);
        }

        // Safety invariant: Do NOT read from the references in this map
        {
            let map_read: &HashMap<NodeIndex, &'static mut GNode<Block, u8>> =
                unsafe { &*(&nodes as *const HashMap<NodeIndex, &'static mut GNode<Block, u8>>) };

            for idx in g.node_indices() {
                let node = nodes.get_mut(&idx).unwrap();

                node.outgoing = bump.alloc_slice_fill_iter(
                    g.edges_directed(idx, Outgoing)
                        .map(|e| GEdge {
                            weight: e.weight().strength_loss(),
                            node: map_read[&e.target()],
                        })
                        .collect_vec()
                        .into_iter(),
                );
                node.incoming_rear = bump.alloc_slice_fill_iter(
                    g.edges_directed(idx, Incoming)
                        .filter(|e| !e.weight().is_side())
                        .map(|e| GEdge {
                            weight: e.weight().strength_loss(),
                            node: map_read[&e.source()],
                        })
                        .collect_vec()
                        .into_iter(),
                );
                node.incoming_side = bump.alloc_slice_fill_iter(
                    g.edges_directed(idx, Incoming)
                        .filter(|e| e.weight().is_side())
                        .map(|e| GEdge {
                            weight: e.weight().strength_loss(),
                            node: map_read[&e.source()],
                        })
                        .collect_vec()
                        .into_iter(),
                );
            }
        }
        // Safety invariant holds until here. We can now read from the references in map because map_read no longer exists
        // We now just have multiple read-only references
        let nodes: HashMap<NodeIndex, &'static GNode<Block, u8>> = unsafe { mem::transmute(nodes) };

        for (idx, block_ref) in nodes {
            callback(&g[idx], block_ref);
        }

        FastGraph {
            phantom: Default::default(),
        }
    }
}

impl<N: 'static, E: 'static> GNode<N, E> {
    pub fn outgoing_edges(&self) -> &'static [GEdge<N, E>] {
        self.outgoing
    }

    pub fn outgoing_neighbours(&self) -> impl Iterator<Item = &'static GNode<N, E>> {
        self.outgoing.iter().map(|e| e.node)
    }

    pub fn incoming_rear_edges(&self) -> &'static [GEdge<N, E>] {
        self.incoming_rear
    }

    pub fn incoming_side_edges(&self) -> &'static [GEdge<N, E>] {
        self.incoming_side
    }
}
