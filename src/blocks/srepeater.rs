use crate::blocks::{Block, OutputPower, ToBlock, Updatable};
use crate::world::graph::GNode;
use crate::world::UpdatableList;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

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
    fn to_block(&self) -> Block {
        Block::SRepeater(SRepeater {
            powered: AtomicBool::new(self.powered),
            last_update: AtomicUsize::new(usize::MAX),
        })
    }
}

#[derive(Debug)]
pub struct SRepeater {
    /// Whether the repeater is currently powered.
    powered: AtomicBool,

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

impl Updatable for SRepeater {
    #[inline(always)]
    fn update(&self, idx: &'static GNode<Block, u8>, _tick_updatable: &mut UpdatableList) -> bool {
        let s_new = idx
            .incoming_rear
            .iter()
            .any(|e| e.node.weight.output_power().saturating_sub(e.weight) > 0);

        s_new != self.powered.load(Ordering::Relaxed)
    }

    fn late_updatable(
        &self,
        _idx: &'static GNode<Block, u8>,
        _updatable: &mut UpdatableList,
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

impl SRepeater {
    pub fn with_power(powered: bool) -> SRepeater {
        SRepeater {
            powered: AtomicBool::new(powered),
            last_update: AtomicUsize::new(usize::MAX),
        }
    }
}
