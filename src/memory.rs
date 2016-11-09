use std::collections::HashMap;
use std::cell::RefCell;

const slabsize: u32 = 256; // 1 KiB == 256 32-bit words

pub struct Memory {
    slabs: HashMap<(u32, u32), Vec<u32>>
}

fn range_for(index: u32) -> ((u32, u32), u32) {
    let diff = index % slabsize;
    let lower_bound = index - diff;
    let upper_bound = lower_bound + slabsize;
    ((lower_bound, upper_bound), diff)
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            slabs: HashMap::new()
        }
    }

    pub fn get(&self, index: u32) -> u32 {
        let (range, diff) = range_for(index);
        match self.slabs.get(&range) {
            Some(x) => x[diff as usize],
            None => 0
        }
    }

    pub fn set(&mut self, index: u32, value: u32) {
        let (range, diff) = range_for(index);
        let vec = self.slabs.entry(range).or_insert_with(|| vec![0; slabsize as usize]);
        vec[diff as usize] = value
    }
}
