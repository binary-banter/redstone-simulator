use crate::blocks::{Block, OutputPower, ToBlock, Updatable};
use crate::world::graph::GNode;
use crate::world::{TickUpdatableList};
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

#[derive(Clone, Debug)]
pub struct CSRepeater {
    /// Whether the repeater is currently powered.
    powered: bool,
}

impl CSRepeater {
    pub fn with_powered(powered: bool) -> CSRepeater {
        CSRepeater { powered }
    }
}

impl ToBlock for CSRepeater {
    fn to_block(&self, on_inputs: u8) -> Block {
        Block::SRepeater(SRepeater {
            powered: AtomicBool::new(self.powered),
            last_update: AtomicUsize::new(usize::MAX),
            on_inputs: AtomicU8::new(on_inputs),
        })
    }
}

#[derive(Debug)]
pub struct SRepeater {
    /// Whether the repeater is currently powered.
    powered: AtomicBool,
    on_inputs: AtomicU8,
    last_update: AtomicUsize,
}

impl OutputPower for SRepeater {
    fn output_power(&self) -> u8 {
        if self.powered.load(Ordering::Relaxed) {
            15
        } else {
            0
        }
    }
}

impl OutputPower for CSRepeater {
    fn output_power(&self) -> u8 {
        if self.powered {
            15
        } else {
            0
        }
    }
}

impl Updatable for SRepeater {
    #[inline(always)]
    fn update(&self, idx: &'static GNode<Block, u8>, tick_updatable: &mut TickUpdatableList, up: bool) -> bool {
        if up{
            //TODO fetch_add
            self.on_inputs.store(self.on_inputs.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
            // assert_eq!(self.on_inputs.load(Ordering::Relaxed) as usize, idx.incoming_rear.iter().filter(|n| n.node.weight.output_power().saturating_sub(n.weight) > 0).count());
            return self.on_inputs.load(Ordering::Relaxed) == 1
        } else {
            //TODO fetch_sub
            self.on_inputs.store(self.on_inputs.load(Ordering::Relaxed) - 1, Ordering::Relaxed);
            // assert_eq!(self.on_inputs.load(Ordering::Relaxed) as usize, idx.incoming_rear.iter().filter(|n| n.node.weight.output_power().saturating_sub(n.weight) > 0).count());
            return self.on_inputs.load(Ordering::Relaxed) == 0
        }
    }

    fn late_update(
        &self,
        idx: &'static GNode<Block, u8>,
        tick_updatable: &mut TickUpdatableList,
        tick_counter: usize,
    ) -> bool {
        if tick_counter == self.last_update.load(Ordering::Relaxed) {
            return false;
        }
        self.last_update.store(tick_counter, Ordering::Relaxed);

        self.powered
            .store(!self.powered.load(Ordering::Relaxed), Ordering::Relaxed);

        true
    }
}
