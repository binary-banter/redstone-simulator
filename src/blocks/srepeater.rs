use std::cell::Cell;
use crate::blocks::{Block, OutputPower, ToBlock, Updatable};
use crate::world::graph::GNode;
use crate::world::UpdatableList;

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
            powered: Cell::new(self.powered),
            last_update: Cell::new(usize::MAX),
            on_inputs: Cell::new(on_inputs),
        })
    }
}

#[derive(Debug)]
pub struct SRepeater {
    /// Whether the repeater is currently powered.
    pub powered: Cell<bool>,
    pub on_inputs: Cell<u8>,
    pub last_update: Cell<usize>,
}

impl OutputPower for SRepeater {
    fn output_power(&self) -> u8 {
        if self.powered.get() {
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
            self.on_inputs.set(
                self.on_inputs.get() + 1,
            );
            self.on_inputs.get() == 1
        } else {
            self.on_inputs.set(
                self.on_inputs.get() - 1,
                
            );
            self.on_inputs.get() == 0
        }
    }

    fn late_update(
        &self,
        _idx: &'static GNode<Block, u8>,
        _tick_updatable: &mut UpdatableList,
        tick_counter: usize,
    ) -> Option<(u8, u8)> {
        if tick_counter == self.last_update.get() {
            return None;
        }
        self.last_update.set(tick_counter, );

        self.powered
            .set(!self.powered.get());

        if self.powered.get() {
            Some((0, 15))
        } else {
            Some((15, 0))
        }
    }
}
