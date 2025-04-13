use core::ops::{Index, IndexMut};
use bitvec::array::BitArray;
use bitvec::bitarr;
use bitvec::order::Lsb0;
use mork_common::constants::MAX_CNODE_SIZE;
use mork_common::types::Array;
use crate::cap::{CNodeCap, Cap};

pub type CapIndex = usize;

#[repr(C, align(4096))]
pub struct CapNode {
    slots: Array<Cap, { MAX_CNODE_SIZE }>,
    usage: BitArray<[usize; bitvec::mem::elts::<usize>(MAX_CNODE_SIZE)], Lsb0>,
}

impl CapNode {
    pub fn new() -> Self {
        let mut usage = bitarr![0; MAX_CNODE_SIZE];
        // slot 0 reserved
        usage.set(0, true);
        Self {
            slots: Array::default(),
            usage,
        }
    }

    pub fn from_cap(cap: &CNodeCap) -> &mut Self {
        unsafe {
            &mut *((cap.base_ptr() << 12) as usize as *mut Self)
        }
    }

    pub fn get_ptr(&self) -> usize {
        self as *const _ as usize
    }

    pub fn alloc_free(&mut self) -> Option<usize> {
        if let Some(slot) = self.usage.first_zero() {
            self.usage.set(slot, true);
            return Some(slot);
        }
        None
    }

    pub fn free_slot(&mut self, index: CapIndex) {
        self[index].free();
        self.usage.set(index, false);
    }

    pub fn is_used(&self, index: usize) -> bool {
        self.usage[index]
    }

    pub fn empty(&self) -> bool {
        self.usage.is_empty()
    }
}

impl Index<usize> for CapNode {
    type Output = Cap;

    fn index(&self, index: usize) -> &Self::Output {
        &self.slots[index]
    }
}

impl IndexMut<usize> for CapNode{

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.usage.set(index, true);
        &mut self.slots[index]
    }
}