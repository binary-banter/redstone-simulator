use crate::block::Block;
use crate::world::World;
use petgraph::prelude::*;
use petgraph::stable_graph::NodeIndex;
use petgraph::{Incoming, Outgoing};
use std::mem;

impl World {
    pub fn step(&mut self) {
        let mut tick_updatable: Vec<NodeIndex> = mem::take(&mut self.updatable);

        while let Some(idx) = tick_updatable.pop() {
            let mut block = self.blocks[idx].clone();

            match &mut block {
                Block::Redstone(s) => {
                    let s_new = self
                        .blocks
                        .edges_directed(idx, Incoming)
                        .map(|edge| {
                            self.blocks[edge.source()]
                                .output_power()
                                .saturating_sub(*edge.weight())
                        })
                        .max()
                        .unwrap_or(0);

                    if *s != s_new {
                        *s = s_new;
                        tick_updatable.extend(self.blocks.neighbors_directed(idx, Outgoing));
                    }
                }
                Block::Solid(s) => {
                    let s_new = self
                        .blocks
                        .edges_directed(idx, Incoming)
                        .map(|edge| {
                            self.blocks[edge.source()]
                                .output_power()
                                .saturating_sub(*edge.weight())
                        })
                        .max()
                        .unwrap_or(0);

                    if *s != s_new {
                        *s = s_new;
                        tick_updatable.extend(self.blocks.neighbors_directed(idx, Outgoing));
                    }
                }
            }

            self.blocks[idx] = block;
        }
    }

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &t in &self.triggers {
            self.blocks[t] = Block::Solid(15);
            for n in self.blocks.neighbors_directed(t, Outgoing) {
                self.updatable.push(n);
            }
        }

        self.step();

        // take redstone power off triggers
        for &t in &self.triggers {
            self.blocks[t] = Block::Solid(0);
            for n in self.blocks.neighbors_directed(t, Outgoing) {
                self.updatable.push(n);
            }
        }
    }
}
