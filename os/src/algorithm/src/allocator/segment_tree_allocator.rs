// Segment Tree Allocator Implement

use super::Allocator;
use alloc::{vec, vec::Vec};
use bit_field::BitArray;

pub struct SegmentTreeAllocator {
    tree: Vec<u8>,
}

impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        leaf_count = capacity.next_power_of_two();
        let mut tree = vec![0u8; leaf_count * 2];

        for index in (capacity..capacity+8){
            tree.set_bit(index, 1);
        }

        for index in (1..leaf_count).rev() {
            let value = tree.get_bit(2 * index) && tree.get_bit(2 * index + 1) && tree.get_bit(index);
            tree.set_bit(index, value);
        }

        SegmentTreeAllocator{tree}

    }

    fn alloc(&mut self) -> Option<usize> {
        let mut index = 1;
        if self.tree.get_bit(index) {
            None
        }else{
            while index < self.tree.len(){
                if(!self.tree.get_bit(index * 2)){
                    index *= 2;
                }else if(!self.tree.get_bit(index * 2 + 1)){
                    index *= 2 + 1;
                }else {
                    panic!("Damaged Segement Tree!");
                }
            }
        }
        self.uploadNode(index, 1);
        Some(index - self.tree.len());
      }

    fn dealloc(&mut self, index: usize) {
        let node = index + self.tree.len()/2;
        self.uploadNode(node, 0);
    }
}

impl SegmentTreeAllocator{

    fn uploadNode(&mut self, mut index: usize, value: bool){
        self.tree.set_bit(index, value);
        while index > 1 {
            index /= 2;
            v = self.get_bit(2 * index) && self.get_bit(2 * index + 1);
            self.tree.set_bit(index, v);
        }
    }
}