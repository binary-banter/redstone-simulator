use crate::blocks::{Block, BlockTrait, BlockTraitLate};
use crate::world::World;
use itertools::Itertools;
use std::cmp::Ordering;
use std::mem;

// impl World {
//     pub fn step(&mut self) {
//         let mut tick_updatable = mem::take(&mut self.updatable);
//         tick_updatable.sort_by(|&x, &y| match (&self.data[x], &self.data[y]) {
//             (
//                 Block::Redstone(Redstone { signal: s1, .. }),
//                 Block::Redstone(Redstone { signal: s2, .. }),
//             ) => s1.cmp(s2),
//             (Block::Redstone(_), _) => Ordering::Less,
//             (_, _) => Ordering::Greater,
//         });
//
//         while let Some(p) = tick_updatable.pop() {
//             let mut block = self.data[p].clone();
//
//             if match block {
//                 Block::Solid(ref mut v) => v.update(p, &self.data, &mut tick_updatable),
//                 Block::Redstone(ref mut v) => v.update(p, &self.data, &mut tick_updatable),
//                 Block::RedstoneBlock => continue,
//                 Block::Trigger(ref mut v) => v.update(p, &self.data, &mut tick_updatable),
//                 Block::Repeater(ref mut v) => v.update(p, &self.data, &mut tick_updatable),
//                 Block::Air => continue,
//                 Block::Torch(ref mut v) => v.update(p, &self.data, &mut tick_updatable),
//                 Block::Comparator(ref mut v) => v.update(p, &self.data, &mut tick_updatable),
//             } {
//                 self.updatable.push(p);
//             }
//
//             self.data[p] = block;
//         }
//
//         // perform end-of-tick updates.
//         for &p in self.updatable.clone().iter().unique() {
//             let mut block = self.data[p].clone();
//
//             match block {
//                 Block::Repeater(ref mut v) => v.update_late(p, &self.data, &mut self.updatable),
//                 Block::Comparator(ref mut v) => v.update_late(p, &self.data, &mut self.updatable),
//                 Block::Torch(ref mut v) => v.update_late(p, &self.data, &mut self.updatable),
//                 _ => {}
//             };
//
//             self.data[p] = block;
//         }
//     }
//
//     pub fn step_with_trigger(&mut self) {
//         // put redstone power on triggers
//         for &t in &self.triggers {
//             self.data[t] = Block::Trigger(Trigger { powered: true });
//             for n in self.data.neighbours(t) {
//                 self.updatable.push(n);
//             }
//         }
//
//         self.step();
//
//         // take redstone power off triggers
//         for &t in &self.triggers {
//             self.data[t] = Block::Trigger(Trigger { powered: false });
//             for n in self.data.neighbours(t) {
//                 self.updatable.push(n);
//             }
//         }
//     }
// }
