use crate::mch_state::MchState;
use std::io::stdin;
use std::io::Read;

pub trait IoDevice {
    fn input(&mut self, regs: (u32, u32, u32));
    fn output(&mut self, regs: (u32, u32, u32));
}
impl IoDevice for MchState {
    fn input(&mut self, regs: (u32, u32, u32)){
        match stdin().bytes().next() {
            Some(value) => {
                self.regs[regs.2 as usize] = value.unwrap() as u32;
                assert!(self.regs[regs.2 as usize] < 255);
            }
            None => self.regs[regs.2 as usize] = u32::MAX,
        }
    }
    fn output(&mut self, regs: (u32, u32, u32)){
        assert!(self.regs[regs.2 as usize] < 255);
        print!("{}", (self.regs[regs.2 as usize] as u8) as char);
    }
}