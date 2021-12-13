use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

use crate::bitpack::newu;
use crate::bitpack::getu;
use crate::io_device::IoDevice;

pub struct MchState {
	pub regs: [u32; 8],
	pub prog_cntr: u32,
	pub addr_space: HashMap<u32, Vec<u32>>
}

pub trait UmFunctions {
    fn exec_cycle(&mut self);
    fn new(init_seg: Vec<u32>);
}

pub trait UmOperations {
    fn cond_move(&mut self, regs: (u32, u32, u32));
    fn seg_load(&mut self, regs: (u32, u32, u32));
    fn seg_store(&mut self, regs: (u32, u32, u32));
    fn add(&mut self, regs: (u32, u32, u32));
    fn multiply(&mut self, regs: (u32, u32, u32));
    fn divide(&mut self, regs: (u32, u32, u32));
    fn nand(&mut self, regs: (u32, u32, u32));
    fn halt(&mut self);
    fn map_seg(&mut self, regs: (u32, u32, u32));
    fn unmap_seg(&mut self, regs: (u32, u32, u32));
    fn load_prog(&mut self, regs: (u32, u32, u32));
    fn load_val(&mut self, reg: u32, val: u32);
}

impl UmFunctions for MchState {
    fn exec_cycle(&mut self) {
        while true {
            let inst: u32 = self.addr_space.get(&(0 as u32)).unwrap()[self.prog_cntr as usize];
            let opcode = getu(inst as u64, 4, 28) as u32;
            let regs = (getu(inst as u64, 3, 6) as u32, getu(inst as u64, 3, 3) as u32, getu(inst as u64, 3, 0) as u32);
            /*if regs.0 == 6 || regs.1 == 6 || regs.2 == 6 {
                println!("{} opcode: {}", self.regs[6], opcode);
            }*/
            println!("{0} {1} {2} {3} {4} {5} {6} {7}", self.regs[0], self.regs[1], self.regs[2], self.regs[3], self.regs[4], self.regs[5], self.regs[6], self.regs[7]);
            match opcode {
                0 => println!("cond move: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                1 => println!("seg load: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                2 => println!("seg store: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                3 => println!("add: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                4 => println!("multiply: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                5 => println!("divide: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                6 => println!("nand: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                7 => println!("halt: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                8 => println!("map seg: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                9 => println!("unmap seg: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                10 => println!("output: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                11 => println!("input: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                12 => println!("load prog: {0} {1} | {2} {3} | {4} {5}", regs.0, self.regs[regs.0 as usize], regs.1, self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]),
                13 => println!("load val: {0} {1}", getu(inst as u64, 3, 25) as u32, getu(inst as u64, 25, 0) as u32),
                _ => println!("default")
            }
            print!("\n\n");
            match opcode {
                0 => self.cond_move(regs),
                1 => self.seg_load(regs),
                2 => self.seg_store(regs),
                3 => self.add(regs),
                4 => self.multiply(regs),
                5 => self.divide(regs),
                6 => self.nand(regs),
                7 => self.halt(),
                8 => self.map_seg(regs),
                9 => self.unmap_seg(regs),
                10 => self.output(regs),
                11 => self.input(regs),
                12 => self.load_prog(regs),
                13 => self.load_val(getu(inst as u64, 3, 25) as u32, getu(inst as u64, 25, 0) as u32),
                _ => println!("default")
            }
            if opcode != 12 {
                self.prog_cntr = self.prog_cntr + 1;
            }
            if self.prog_cntr as usize == (self.addr_space.get(&(0 as u32)).unwrap().len() - 1) {
                self.halt();
            }
        }
    }
    fn new(init_seg: Vec<u32>) {
        let r: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u32>> = HashMap::new();
        mem.insert(0 as u32, init_seg);
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };
        machine.exec_cycle();
    }
}

impl UmOperations for MchState {
    fn cond_move(&mut self, regs: (u32, u32, u32)){
        if self.regs[regs.2 as usize] != (0 as u32) {
            self.regs[regs.0 as usize] = self.regs[regs.1 as usize];
        }
    }
    fn seg_load(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = self.addr_space.get(&self.regs[regs.1 as usize]).unwrap()[regs.2 as usize];
    }
    fn seg_store(&mut self, regs: (u32, u32, u32)){
        let mut temp_vec: Vec<u32> = self.addr_space.get(&self.regs[regs.0 as usize]).unwrap().to_vec();
        temp_vec[self.regs[regs.1 as usize] as usize] = self.regs[regs.2 as usize];
        self.addr_space.insert(self.regs[regs.0 as usize], temp_vec);
    }
    fn add(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = (((self.regs[regs.1 as usize] as u64 + self.regs[regs.2 as usize] as u64) << 32) >> 32) as u32;
    }
    fn multiply(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = (((self.regs[regs.1 as usize] as u64 * self.regs[regs.2 as usize] as u64) << 32) >> 32) as u32;
    }
    fn divide(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = self.regs[regs.1 as usize] / self.regs[regs.2 as usize];
    }
    fn nand(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = !(self.regs[regs.1 as usize] & self.regs[regs.2 as usize]);
    }
    fn halt(&mut self){
        std::process::exit(0x0000);
    }
    fn map_seg(&mut self, regs: (u32, u32, u32)){
        let segment: Vec<u32> = vec![0 as u32; self.regs[(regs.2) as usize] as usize];
        let unused_key = (0..u32::MAX).into_iter().find(|key| !self.addr_space.contains_key(key)).unwrap();
        self.addr_space.insert(unused_key, segment);
        self.regs[regs.1 as usize] = unused_key;
    }
    fn unmap_seg(&mut self, regs: (u32, u32, u32)){
        self.addr_space.remove(&self.regs[regs.2 as usize]);
    }
    fn load_prog(&mut self, regs: (u32, u32, u32)){
        if self.regs[regs.1 as usize] != 0 {
            let mut new_seg = self.addr_space.get(&self.regs[regs.1 as usize]).unwrap().to_vec();
            self.addr_space.remove(&(0 as u32));
            self.addr_space.insert(0 as u32, new_seg);
        }
        self.prog_cntr = self.regs[regs.2 as usize];
    }
    fn load_val(&mut self, reg: u32, val: u32){
        self.regs[reg as usize] = val;
    }
}

#[cfg(test)]
mod tests {

    use crate::bitpack::getu;
    use crate::bitpack::newu;
    use crate::mch_state::MchState;
    use crate::mch_state::UmOperations;
    use std::collections::HashMap;

    #[test]
    fn cond_move_test() {
        let r: [u32; 8] = [5, 0, 1, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.cond_move(regs);
        assert!(machine.regs[0] == machine.regs[1]);
        machine.regs[0] = 5;
        machine.regs[2] = 0;
        machine.cond_move(regs);
        assert!(machine.regs[0] == 5);
    }
    #[test]
    fn seg_load_test() {
        let r: [u32; 8] = [5, 1, 3, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u32>> = HashMap::new();

        let test_seg: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        mem.insert(1 as u32, test_seg);

        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.seg_load(regs);
        assert!(machine.regs[0] == 2 as u32);
    }
    #[test]
    fn seg_store_test() {
        let r: [u32; 8] = [1, 1, 6, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u32>> = HashMap::new();

        let test_seg: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        mem.insert(1 as u32, test_seg);

        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.seg_store(regs);
        assert!(machine.addr_space.get(&(1 as u32)).unwrap()[1] == 6 as u32);
    }
    #[test]
    fn add_test() {
        let r: [u32; 8] = [5, 1, 1, u32::MAX, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.add(regs);
        assert!(machine.regs[0] == 2 as u32);
        let mut regs = (0 as u32, 2 as u32, 3 as u32);
        machine.add(regs);
        println!("{}", machine.regs[0]);
        assert!(machine.regs[0] == 0 as u32);
    }
    #[test]
    fn multiply_test() {
        let r: [u32; 8] = [5, 2, 2, u32::MAX, 2, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.multiply(regs);
        assert!(machine.regs[0] == 4 as u32);
        let mut regs = (0 as u32, 2 as u32, 3 as u32);
        machine.multiply(regs);
        println!("{}", machine.regs[0]);
        assert!(machine.regs[0] == 4294967294 as u32);
    }
    #[test]
    fn division_test() {
        let r: [u32; 8] = [5, 2, 2, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.divide(regs);
        assert!(machine.regs[0] == 1 as u32);
    }
    #[test]
    fn nand_test() {
        let r: [u32; 8] = [5, u32::MAX, u32::MAX, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.nand(regs);
        assert!(machine.regs[0] == 0 as u32);
    }
    #[test]
    fn map_seg_test() {
        let r: [u32; 8] = [5, 0, 10, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let test_seg: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        mem.insert(0 as u32, test_seg);
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.map_seg(regs);
        assert!(machine.regs[1] == 1 as u32);
        assert!(machine.addr_space.get(&(1 as u32)).unwrap()[0] == 0);
    }
    #[test]
    fn unmap_seg_test() {
        let r: [u32; 8] = [5, 0, 10, 1, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let test_seg: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        mem.insert(0 as u32, test_seg);
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.map_seg(regs);
        regs = (0 as u32, 1 as u32, 3 as u32);
        machine.unmap_seg(regs);
        let x = machine.addr_space.get(&(1 as u32));
        match x {
            Some(_) => assert!(1 == 2),
            None => assert!(1 == 1)
        }
    }
    #[test]
    fn load_prog_test() {
        let r: [u32; 8] = [5, 1, 4, 1, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let test_seg: Vec<u32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        mem.insert(0 as u32, test_seg);
        let test_seg2: Vec<u32> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        mem.insert(1 as u32, test_seg2);
        let test_seg3: Vec<u32> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.load_prog(regs);
        
        let x = machine.addr_space.get(&(0 as u32)).unwrap();
        for i in 0..test_seg3.len() {
            assert!(x[i] == test_seg3[i]);
        }
        assert!(machine.prog_cntr == 4 as u32);
    }
    #[test]
    fn load_val_test() {
        let r: [u32; 8] = [5, 0, 1, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let reg: u32 = 0;
        let value: u32 = 7;
        machine.load_val(reg, value);
        assert!(machine.regs[0] == value);
    }
}