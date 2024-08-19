pub const MEMORY_SIZE: usize = 0x10000; // 64kB

/// Random-Access Memory (RAM)
#[derive(Debug)]
pub struct Memory {
    data: Box<[u8; MEMORY_SIZE]>,
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&self, addr: u16) -> u8 {
        *self
            .data
            .get(addr as usize)
            .expect("ERROR(memory::Ram): `addr` out of bounds")
    }

    pub fn read_slice(&self, start: u16, offset: u16) -> &[u8] {
        self.data
            .get((start as usize)..(start as usize + offset as usize))
            .expect("ERROR(memory::Ram): slice out of bounds")
    }

    pub fn get_mut(&mut self, addr: u16) -> &mut u8 {
        self.data
            .get_mut(addr as usize)
            .expect("ERROR(memory::Ram): `addr` out of bounds")
    }

    pub fn write(&mut self, addr: u16, val: u8) -> u8 {
        let old = self.read(addr);
        self.data[addr as usize] = val;
        old
    }

    pub fn write_slice(&mut self, start: u16, data: &[u8]) -> Result<(), &'static str> {
        for (i, &val) in data.iter().enumerate() {
            if start as usize + i > MEMORY_SIZE {
                return Err("data does not fit into memory");
            }
            self.data[start as usize + i] = val;
        }

        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            data: Box::new([0; MEMORY_SIZE]),
        }
    }
}
