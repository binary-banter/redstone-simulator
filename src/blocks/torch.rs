use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, InputSide, OutputPower, ToBlock, Updatable};
use crate::world::graph::GNode;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::world::UpdatableList;

#[derive(Debug)]
pub struct Torch {
    /// Whether the torch is currently lit.
    lit: AtomicBool,

    last_update: AtomicUsize,
}

#[derive(Copy, Clone, Debug)]
pub struct CTorch {
    /// Whether the torch is currently lit.
    lit: bool,

    /// Direction the torch points in.
    facing: Facing,
}

impl Default for Torch {
    fn default() -> Self {
        Torch {
            lit: AtomicBool::new(true),
            last_update: AtomicUsize::new(usize::MAX),
        }
    }
}

impl OutputPower for Torch {
    fn output_power(&self) -> u8 {
        if self.lit.load(Ordering::Relaxed) {
            15
        } else {
            0
        }
    }
}

impl OutputPower for CTorch {
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
            lit: AtomicBool::new(lit),
            last_update: AtomicUsize::new(usize::MAX),
        }
    }
}

impl BlockConnections for CTorch {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, facing: Facing) -> Option<InputSide> {
        if self.facing == facing {
            Some(InputSide::Rear)
        } else {
            None
        }
    }
}
impl ToBlock for CTorch {
    fn to_block(&self, on_inputs: u8) -> Block {
        Block::Torch(Torch {
            lit: AtomicBool::new(self.lit),
            last_update: AtomicUsize::new(usize::MAX),
        })
    }
}

impl Updatable for Torch {
    #[inline(always)]
    fn update(&self, idx: &'static GNode<Block, u8>, tick_updatable: &mut UpdatableList, up: bool) -> bool {
        let s_new = idx
            .incoming_rear
            .iter()
            .any(|e| e.node.weight.output_power().saturating_sub(e.weight) > 0);

        s_new == self.lit.load(Ordering::Relaxed)
    }

    fn late_update(
        &self,
        idx: &'static GNode<Block, u8>,
        tick_updatable: &mut UpdatableList,
        tick_counter: usize,
    ) -> bool {
        if tick_counter == self.last_update.load(Ordering::Relaxed) {
            return false;
        }
        self.last_update.store(tick_counter, Ordering::Relaxed);

        self.lit
            .store(!self.lit.load(Ordering::Relaxed), Ordering::Relaxed);

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

        CTorch { lit, facing }
    }
}
