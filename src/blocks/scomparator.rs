use crate::blocks::{Block, OutputPower, ToBlock, Updatable};
use crate::world::graph::GNode;
use crate::world::UpdatableList;

#[derive(Debug)]
pub struct SComparator {

}

#[derive(Clone, Debug)]
pub struct CSComparator {
    pub rear: u8,
    pub side: u8,
}

impl OutputPower for SComparator{
    fn output_power(&self) -> u8 {
        todo!()
    }
}

impl OutputPower for CSComparator{
    fn output_power(&self) -> u8 {
        0 //TODO?
    }
}

impl ToBlock for CSComparator{
    fn to_block(&self, on_inputs: u8) -> Block {
        Block::SComparator(SComparator {

        })
    }
}

impl Updatable for SComparator {
    #[inline(always)]
    fn update(&self, _idx: &'static GNode<Block, u8>, _tick_updatable: &mut UpdatableList, up: bool, ) -> bool {
        todo!()
    }

    fn late_update(
        &self,
        _idx: &'static GNode<Block, u8>,
        _tick_updatable: &mut UpdatableList,
        tick_counter: usize,
    ) -> Option<(u8, u8)> {
        todo!()
    }
}