use crate::blocks::{Block, OutputPower, ToBlock, Updatable};
use crate::world::graph::GNode;
use crate::world::UpdatableList;
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
    pub powered: AtomicBool,
    pub on_inputs: AtomicU8,
    pub last_update: AtomicUsize,
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
    fn update(
        &self,
        _idx: &'static GNode<Block, u8>,
        _tick_updatable: &mut UpdatableList,
        up: bool,
    ) -> bool {
        if up {
            self.on_inputs.store(
                self.on_inputs.load(Ordering::Relaxed) + 1,
                Ordering::Relaxed,
            );
            self.on_inputs.load(Ordering::Relaxed) == 1
        } else {
            self.on_inputs.store(
                self.on_inputs.load(Ordering::Relaxed) - 1,
                Ordering::Relaxed,
            );
            self.on_inputs.load(Ordering::Relaxed) == 0
        }
    }

    fn late_update(
        &self,
        _idx: &'static GNode<Block, u8>,
        _tick_updatable: &mut UpdatableList,
        tick_counter: usize,
    ) -> Option<(u8, u8)> {
        if tick_counter == self.last_update.load(Ordering::Relaxed) {
            return None;
        }
        self.last_update.store(tick_counter, Ordering::Relaxed);

        self.powered
            .store(!self.powered.load(Ordering::Relaxed), Ordering::Relaxed);

        if self.powered.load(Ordering::Relaxed){
            Some((0, 15))
        } else{
            Some((15, 0))
        }
    }
}
