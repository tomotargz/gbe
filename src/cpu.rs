#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}
pub struct Cpu {
    regs: Registers,
    ctx: Ctx,
}
impl Cpu {
    pub fn fetch(&mut self, bus: &Periferals) {
        self.ctx.opcode = bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        self.ctx.cb = false;
    }
    pub fn decode(&mut self, bus: &mut Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _ => panic!("Not implemented: {:02x}", self.opcode),
        }
    }
    pub fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }
}
