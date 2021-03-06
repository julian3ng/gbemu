use ::mmu::MMU;


#[derive(PartialEq, Debug)]
struct Clock { m: u32, t: u32 }

impl Clock {
    fn set(&mut self, m: u32, t: u32) {
        assert!(m*4 == t);
        self.m = m;
        self.t = t;
    }

    fn update(&mut self, m: u32, t: u32) {
        assert!(m*4 == t);
        self.m += m;
        self.t += t;
    }
}

pub struct CPU {
    a: u8, f: u8,
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8,
    sp: u16,
    pc: u16,
    last_clock_tick: Clock,
    clock: Clock,
    mmu: MMU
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            sp: 0, pc: 0,
            last_clock_tick: Clock { m: 0, t: 0 },
            clock: Clock { m: 0, t: 0},
            mmu: MMU::new()
        }
    }

    pub fn reset(&mut self) {
        self.a = 0; self.f = 0;
        self.b = 0; self.c = 0;
        self.d = 0; self.e = 0;
        self.h = 0; self.l = 0;
        self.sp = 0;
        self.pc = 0;
        self.last_clock_tick.set(0, 0);
        self.clock.set(0, 0);
    }

    fn a(&self) -> u8 { self.a }
    fn f(&self) -> u8 { self.f }
    fn b(&self) -> u8 { self.b }    
    fn c(&self) -> u8 { self.c }
    fn d(&self) -> u8 { self.d }
    fn e(&self) -> u8 { self.e }
    fn h(&self) -> u8 { self.h }
    fn l(&self) -> u8 { self.l }

    fn af(&self) -> u16 { ((self.a as u16) << 8) | (self.f as u16) }
    fn bc(&self) -> u16 { ((self.b as u16) << 8) | (self.c as u16) }
    fn de(&self) -> u16 { ((self.d as u16) << 8) | (self.e as u16) }
    fn hl(&self) -> u16 { ((self.h as u16) << 8) | (self.l as u16) }

    fn set_a(&mut self, val: u8) { self.a = val }
    fn set_f(&mut self, val: u8) { self.f = val }
    fn set_b(&mut self, val: u8) { self.b = val }        
    fn set_c(&mut self, val: u8) { self.c = val }
    fn set_d(&mut self, val: u8) { self.d = val }
    fn set_e(&mut self, val: u8) { self.e = val }
    fn set_h(&mut self, val: u8) { self.h = val }
    fn set_l(&mut self, val: u8) { self.l = val }

    // LSBs go into first, MSBs go into second
    fn set_af(&mut self, val: u16) {
        self.set_a(((val & 0xFF00) >> 8) as u8);
        self.set_f((val & 0x00FF) as u8);
    }

    fn set_bc(&mut self, val: u16) {
        self.set_b(((val & 0xFF00) >> 8) as u8);
        self.set_c((val & 0x00FF) as u8);
    }

    fn set_de(&mut self, val: u16) {
        self.set_d(((val & 0xFF00) >> 8) as u8);
        self.set_e((val & 0x00FF) as u8);
    }

    fn set_hl(&mut self, val: u16) {
        self.set_h(((val & 0xFF00) >> 8) as u8);
        self.set_l((val & 0x00FF) as u8);
    }    
    
    
    pub fn call(&mut self, opcode: u8) {
        match opcode {
            0x00 => {
                self.last_clock_tick.set(1, 4);
            }
            0x01 => {
                self.c = self.mmu.rb(self.pc);
                self.pc += 1;
                self.b = self.mmu.rb(self.pc);
                self.pc += 1;
                self.last_clock_tick.set(3, 12);
            }
            0x02 => { }
            0x03 => { }
            0x04 => { }
            0x05 => { }
            0x06 => { }
            0x07 => { }
            0x08 => { }
            0x09 => { }
            0x0a => { }
            0x0b => { }
            0x0c => { }
            0x0d => { }
            0x0e => { }
            0x0f => { }

            0x10 => { }
            0x11 => { }
            0x12 => { }
            0x13 => { }
            0x14 => { }
            0x15 => { }
            0x16 => { }
            0x17 => { }
            0x18 => { }
            0x19 => { }
            0x1a => { }
            0x1b => { }
            0x1c => { }
            0x1d => { }
            0x1e => { }
            0x1f => { }

            0x20 => { }
            0x21 => { }
            0x22 => { }
            0x23 => { }
            0x24 => { }
            0x25 => { }
            0x26 => { }
            0x27 => { }
            0x28 => { }
            0x29 => { }
            0x2a => { }
            0x2b => { }
            0x2c => { }
            0x2d => { }
            0x2e => { }
            0x2f => { }

            0x30 => { }
            0x31 => { }
            0x32 => { }
            0x33 => { }
            0x34 => { }
            0x35 => { }
            0x36 => { }
            0x37 => { }
            0x38 => { }
            0x39 => { }
            0x3a => { }
            0x3b => { }
            0x3c => { }
            0x3d => { }
            0x3e => { }
            0x3f => { }

            0x40 => { }
            0x41 => { }
            0x42 => { }
            0x43 => { }
            0x44 => { }
            0x45 => { }
            0x46 => { }
            0x47 => { }
            0x48 => { }
            0x49 => { }
            0x4a => { }
            0x4b => { }
            0x4c => { }
            0x4d => { }
            0x4e => { }
            0x4f => { }

            0x50 => { }
            0x51 => { }
            0x52 => { }
            0x53 => { }
            0x54 => { }
            0x55 => { }
            0x56 => { }
            0x57 => { }
            0x58 => { }
            0x59 => { }
            0x5a => { }
            0x5b => { }
            0x5c => { }
            0x5d => { }
            0x5e => { }
            0x5f => { }

            0x60 => { }
            0x61 => { }
            0x62 => { }
            0x63 => { }
            0x64 => { }
            0x65 => { }
            0x66 => { }
            0x67 => { }
            0x68 => { }
            0x69 => { }
            0x6a => { }
            0x6b => { }
            0x6c => { }
            0x6d => { }
            0x6e => { }
            0x6f => { }

            0x70 => { }
            0x71 => { }
            0x72 => { }
            0x73 => { }
            0x74 => { }
            0x75 => { }
            0x76 => { }
            0x77 => { }
            0x78 => { }
            0x79 => { }
            0x7a => { }
            0x7b => { }
            0x7c => { }
            0x7d => { }
            0x7e => { }
            0x7f => { }

            0x80 => { }
            0x81 => { }
            0x82 => { }
            0x83 => { }
            0x84 => { }
            0x85 => { }
            0x86 => { }
            0x87 => { }
            0x88 => { }
            0x89 => { }
            0x8a => { }
            0x8b => { }
            0x8c => { }
            0x8d => { }
            0x8e => { }
            0x8f => { }

            0x90 => { }
            0x91 => { }
            0x92 => { }
            0x93 => { }
            0x94 => { }
            0x95 => { }
            0x96 => { }
            0x97 => { }
            0x98 => { }
            0x99 => { }
            0x9a => { }
            0x9b => { }
            0x9c => { }
            0x9d => { }
            0x9e => { }
            0x9f => { }

            0xa0 => { }
            0xa1 => { }
            0xa2 => { }
            0xa3 => { }
            0xa4 => { }
            0xa5 => { }
            0xa6 => { }
            0xa7 => { }
            0xa8 => { }
            0xa9 => { }
            0xaa => { }
            0xab => { }
            0xac => { }
            0xad => { }
            0xae => { }
            0xaf => { }

            0xb0 => { }
            0xb1 => { }
            0xb2 => { }
            0xb3 => { }
            0xb4 => { }
            0xb5 => { }
            0xb6 => { }
            0xb7 => { }
            0xb8 => { }
            0xb9 => { }
            0xba => { }
            0xbb => { }
            0xbc => { }
            0xbd => { }
            0xbe => { }
            0xbf => { }

            0xc0 => { }
            0xc1 => { }
            0xc2 => { }
            0xc3 => { }
            0xc4 => { }
            0xc5 => { }
            0xc6 => { }
            0xc7 => { }
            0xc8 => { }
            0xc9 => { }
            0xca => { }
            0xcb => { }
            0xcc => { }
            0xcd => { }
            0xce => { }
            0xcf => { }

            0xd0 => { }
            0xd1 => { }
            0xd2 => { }
            0xd3 => { }
            0xd4 => { }
            0xd5 => { }
            0xd6 => { }
            0xd7 => { }
            0xd8 => { }
            0xd9 => { }
            0xda => { }
            0xdb => { }
            0xdc => { }
            0xdd => { }
            0xde => { }
            0xdf => { }

            0xe0 => { }
            0xe1 => { }
            0xe2 => { }
            0xe3 => { }
            0xe4 => { }
            0xe5 => { }
            0xe6 => { }
            0xe7 => { }
            0xe8 => { }
            0xe9 => { }
            0xea => { }
            0xeb => { }
            0xec => { }
            0xed => { }
            0xee => { }
            0xef => { }

            0xf0 => { }
            0xf1 => { }
            0xf2 => { }
            0xf3 => { }
            0xf4 => { }
            0xf5 => { }
            0xf6 => { }
            0xf7 => { }
            0xf8 => { }
            0xf9 => { }
            0xfa => { }
            0xfb => { }
            0xfc => { }
            0xfd => { }
            0xfe => { }
            0xff => { }
            _ => panic!("Can't happen")
        }
    }

    pub fn tick(&mut self) {
        self.clock.update(self.last_clock_tick.m, self.last_clock_tick.t);        
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.mmu.rb(self.pc);
            println!("{} {:?}", self.pc, self.clock);
            self.pc += 1;
            self.call(opcode);
            // following line exists to keep pc from overflowing in javascript
            // we have u16 type so this isn't necessary
            // self.pc &= 65535; 
            self.tick();
        }
    }
}

#[test]
fn initialization() {
    let cpu = CPU::new();
    assert!(cpu.a() == 0);
    assert!(cpu.f() == 0);
    assert!(cpu.b() == 0);
    assert!(cpu.c() == 0);
    assert!(cpu.d() == 0);
    assert!(cpu.e() == 0);
    assert!(cpu.h() == 0);
    assert!(cpu.l() == 0);
    assert!(cpu.af() == 0);
    assert!(cpu.bc() == 0);
    assert!(cpu.de() == 0);
    assert!(cpu.hl() == 0);
    assert!(cpu.sp == 0);
    assert!(cpu.pc == 0);
    assert!(cpu.last_clock_tick == Clock { m: 0, t: 0 });
    assert!(cpu.clock == Clock { m: 0, t: 0});
}

#[test]
fn setters_and_getters() {
    let mut cpu = CPU::new();
    cpu.set_a(1);
    cpu.set_f(2);
    cpu.set_b(3);
    cpu.set_c(4);
    cpu.set_d(5);
    cpu.set_e(6);
    cpu.set_h(7);
    cpu.set_l(8);
    assert!(cpu.a() == 1);
    assert!(cpu.f() == 2);
    assert!(cpu.b() == 3);    
    assert!(cpu.c() == 4);
    assert!(cpu.d() == 5);
    assert!(cpu.e() == 6);
    assert!(cpu.h() == 7);
    assert!(cpu.l() == 8);    
    assert!(cpu.af() == 0x0102);
    assert!(cpu.bc() == 0x0304);
    assert!(cpu.de() == 0x0506);
    assert!(cpu.hl() == 0x0708);
    cpu.set_af(0x0a0b);
    cpu.set_bc(0x0c0d);
    cpu.set_de(0x0e0f);
    cpu.set_hl(0x0001);
    assert!(cpu.af() == 0x0a0b);
    assert!(cpu.bc() == 0x0c0d);
    assert!(cpu.de() == 0x0e0f);
    assert!(cpu.hl() == 0x0001);
}

#[test]
fn clock() {
    let mut cpu = CPU::new();
    cpu.last_clock_tick.set(1, 4);
    assert!(cpu.last_clock_tick.m == 1);
    assert!(cpu.last_clock_tick.t == 4);
    cpu.clock.update(1, 4);
    cpu.clock.update(3, 12);
    assert!(cpu.clock.m == 4);
    assert!(cpu.clock.t == 16);
}

#[test]
#[should_panic]
fn non_multiple_of_4() {
    let mut cpu = CPU::new();
    cpu.last_clock_tick.set(1, 5);
}
