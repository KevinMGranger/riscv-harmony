use std::collections::HashMap;

const slabsize: u32 = 1024; // 1 KiB == 256 32-bit words

pub struct Memory {
    slabs: HashMap<(u32, u32), Vec<u8>>
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

    pub fn get_byte(&self, index: u32) -> Option<u8> {
        let (range, diff) = range_for(index);
        self.slabs.get(&range).map(|vec| vec[diff as usize])
    }

    pub fn get_half(&self, index: u32) -> Option<u16> {
        let mut val = 0u16;

        for i in 0..2 {
            if let Some(x) = self.get_byte(index + i) {
                val = (x as u16) << i;
            } else {
                return None
            }
        }

        Some(val)
    }

    pub fn get_word(&self, index: u32) -> Option<u32> {
        let mut val = 0u32;

        for i in 0..4 {
            if let Some(x) = self.get_byte(index + i) {
                val = (x as u32) << i;
            } else {
                return None
            }
        }

        Some(val)
    }

    pub fn set_byte(&mut self, index: u32, value: u8) {
        let (range, diff) = range_for(index);
        let mut vec = self.slabs.entry(range).or_insert_with(|| vec![0u8; slabsize as usize]);
        vec[diff as usize] = value
    }

    pub fn set_half(&mut self, index: u32, value: u16) {
        for i in 0..2 {
            let byte = (value >> i) as u8;
            self.set_byte(index + i, byte);
        }
    }

    pub fn set_word(&mut self, index: u32, value: u32) {
        for i in 0..4 {
            let byte = (value >> i) as u8;
            self.set_byte(index + i, byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_get_byte() {
        let mut mem = Memory::new();
        mem.set_byte(0, 1);
        assert_eq!(mem.get_byte(0), Some(1))
    }
}
