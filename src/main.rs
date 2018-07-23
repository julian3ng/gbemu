extern crate gbemu;

use gbemu::cpu;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.run();
}
