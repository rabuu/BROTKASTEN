const RAM_SIZE: usize = 0xffff; // 64kB

#[derive(Debug)]
pub struct Ram {
    data: Box<[u8; RAM_SIZE]>,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            data: Box::new([0; RAM_SIZE]),
        }
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
}
