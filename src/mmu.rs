pub struct MMU {
    
}

impl MMU {
    pub fn new() -> Self {
        MMU { }
    }
    pub fn rb(&self, addr: u16) -> u8 { 0}
    pub fn rw(&self, addr: u16) -> u16 {0 }  
    pub fn wb(&mut self, addr: u16, val: u8) { }
    pub fn ww(&mut self, addr: u16, val: u16) { }
}
