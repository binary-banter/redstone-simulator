use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, InputSide, OutputPower, ToBlock, Updatable};
use crate::world::graph::GNode;
use crate::world::UpdatableList;
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Repeater {
    /// Whether the repeater is currently powered.
    powered: Cell<bool>,

    /// Next power when count reaches the repeater delay.
    next_powered: Cell<bool>,

    locking_signal: Cell<bool>,

    /// The repeater delay in ticks, ranges from 1 to 4 inclusive.
    delay: u8,

    /// Number of ticks passed since a new input signal was detected.
    count: Cell<u8>,

    last_update: Cell<usize>,
}

#[derive(Copy, Clone, Debug)]
pub struct CRepeater {
    /// Whether the repeater is currently powered.
    powered: bool,

    /// The repeater delay in ticks, ranges from 1 to 4 inclusive.
    delay: u8,

    /// Direction of the input side of the repeater.
    facing: Facing,
}

impl OutputPower for Repeater {
    fn output_power(&self) -> u8 {
        if self.powered.get() {
            15
        } else {
            0
        }
    }
}

impl OutputPower for CRepeater {
    fn output_power(&self) -> u8 {
        if self.powered {
            15
        } else {
            0
        }
    }
}

impl BlockConnections for CRepeater {
    fn can_output(&self, facing: Facing) -> bool {
        self.facing == facing.rev()
    }

    fn can_input(&self, facing: Facing) -> Option<InputSide> {
        if self.facing == facing.rotate_left() || self.facing == facing.rotate_right() {
            Some(InputSide::Side)
        } else if self.facing == facing.rev() {
            Some(InputSide::Rear)
        } else {
            None
        }
    }
}
impl ToBlock for CRepeater {
    fn to_block(&self, _on_inputs: u8) -> Block {
        Block::Repeater(Repeater {
            powered: Cell::new(self.powered),
            next_powered: Cell::new(self.powered),
            locking_signal: Cell::new(false),
            delay: self.delay,
            count: Cell::new(0),
            last_update: Cell::new(usize::MAX),
        })
    }
}

impl Updatable for Repeater {
    #[inline(always)]
    fn update(
        &self,
        idx: &'static GNode<Block, u8>,
        tick_updatable: &mut UpdatableList,
        _up: bool,
    ) -> bool {
        let s_new = idx
            .incoming_rear
            .iter()
            .any(|e| e.node.weight.output_power().saturating_sub(e.weight) > 0);

        let locked_now = idx
            .incoming_side
            .iter()
            .any(|e| e.node.weight.output_power().saturating_sub(e.weight) > 0);

        if locked_now {
            return false;
        }

        let locked_next_tick = idx.incoming_side.iter().any(|e| e.node.weight.will_lock());

        if locked_next_tick == self.locking_signal.get() {
            tick_updatable.extend(
                idx.outgoing_neighbours()
                    .filter(|b| matches!(b.weight, Block::Repeater(_))), //TODO filter may not be needed if we pre-compute
            );
        }

        // if signal strength has changed, update neighbours
        match (
            s_new,
            self.next_powered.get() == s_new,
            self.count.get() == 0,
        ) {
            // Signal changed upwards: update next signal and reset count.
            (true, false, _) => {
                self.next_powered.set(s_new);
                self.count.set(0);
            }
            // Signal changed downward, and is not propagating already: update next signal.
            (false, false, true) => {
                self.next_powered.set(s_new);
            }
            // Other cases.
            (_, _, _) => {}
        };

        self.locking_signal.set(if locked_next_tick {
            self.powered.get()
        } else if self.count.get() + 1 == self.delay {
            self.next_powered.get()
        } else {
            self.powered.get()
        });

        self.powered.get() != self.next_powered.get()
    }

    fn late_update(
        &self,
        idx: &'static GNode<Block, u8>,
        tick_updatable: &mut UpdatableList,
        tick_counter: usize,
    ) -> Option<(u8, u8)> {
        if tick_counter == self.last_update.get() {
            return None;
        }
        self.last_update.set(tick_counter);

        self.count.set(self.count.get() + 1);
        tick_updatable.push(idx);
        if self.count.get() == self.delay {
            self.count.set(0);
            self.powered.set(self.locking_signal.get());
            if self.powered.get() {
                Some((0, 15))
            } else {
                Some((15, 0))
            }
        } else {
            None
        }
    }
}

impl Repeater {
    pub fn will_lock(&self) -> bool {
        self.locking_signal.get()
    }
}

impl CRepeater {
    pub fn delay(&self) -> u8 {
        self.delay
    }

    pub fn facing(&self) -> Facing {
        self.facing
    }

    pub fn is_powered(&self) -> bool {
        self.powered
    }
}

impl From<HashMap<&str, &str>> for CRepeater {
    fn from(meta: HashMap<&str, &str>) -> Self {
        let powered = meta.get("powered").map(|&x| x == "true").unwrap();

        CRepeater {
            powered,
            facing: Facing::from(meta["facing"]),
            delay: meta["delay"].parse().unwrap(),
        }
    }
}
