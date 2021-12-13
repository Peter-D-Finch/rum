use std::collections::HashMap;
use bitpack::bitpack::getu;
use std::io::stdin;
use std::io::Read;

pub struct MchState {
	regs: [u32; 8],
	prog_cntr: u32,
	addr_space: HashMap<u32, Vec<u64>>
}

pub trait UmFunctions {
    fn exec_cycle(&mut self);
    fn new(init_seg: Vec<u64>);
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

pub trait IoDevice {
    fn input(&mut self, regs: (u32, u32, u32));
    fn output(&mut self, regs: (u32, u32, u32));
}

impl UmFunctions for MchState {
    fn exec_cycle(&mut self) {
        let inst: u64 = self.addr_space.get(&(0 as u32)).unwrap()[self.prog_cntr as usize];
        let opcode = getu(inst, 4, 28);
        let regs = (getu(inst, 3, 6) as u32, getu(inst, 3, 3) as u32, getu(inst, 3, 0) as u32);
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
            13 => self.load_val(getu(inst, 3, 25) as u32, getu(inst, 25, 0) as u32),
            _ => println!("default")
        }
        /*match opcode {
            0 => println!("cond move: {0} {1} {2}", regs.0, regs.1, regs.2),
            1 => println!("seg load: {0} {1} {2}", regs.0, regs.1, regs.2),
            2 => println!("seg store: {0} {1} {2}", regs.0, regs.1, regs.2),
            3 => println!("add: {0} {1} {2}", regs.0, regs.1, regs.2),
            4 => println!("multiply: {0} {1} {2}", regs.0, regs.1, regs.2),
            5 => println!("divide: {0} {1} {2}", regs.0, regs.1, regs.2),
            6 => println!("nand: {0} {1} {2}", regs.0, regs.1, regs.2),
            7 => println!("halt: {0} {1} {2}", regs.0, regs.1, regs.2),
            8 => println!("map seg: {0} {1} {2}", regs.0, regs.1, regs.2),
            9 => println!("unmap seg: {0} {1} {2}", regs.0, regs.1, regs.2),
            10 => println!("output: {0} {1} {2}", regs.0, regs.1, regs.2),
            11 => println!("input: {0} {1} {2}", regs.0, regs.1, regs.2),
            12 => println!("load prog: {0} {1} {2}", regs.0, regs.1, regs.2),
            13 => println!("load val: {0} {1}", getu(inst, 3, 25) as u32, getu(inst, 25, 0) as u32),
            _ => println!("default")
        }*/
        if opcode != 12 {
            self.prog_cntr = self.prog_cntr + 1;
        }
        if self.prog_cntr as usize == (self.addr_space.get(&(0 as u32)).unwrap().len() - 1) {
            self.halt();
        }
        else {
            self.exec_cycle();
        }
    }
    fn new(init_seg: Vec<u64>) {
        let r: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u64>> = HashMap::new();
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
        self.regs[regs.0 as usize] = self.addr_space.get(&self.regs[regs.1 as usize]).unwrap()[regs.2 as usize] as u32;
    }
    fn seg_store(&mut self, regs: (u32, u32, u32)){
        let mut temp_vec: Vec<u64> = self.addr_space.get(&self.regs[regs.0 as usize]).unwrap().to_vec();
        temp_vec[regs.1 as usize] = self.regs[regs.2 as usize] as u64;
        self.addr_space.insert(self.regs[regs.0 as usize], temp_vec);
    }
    fn add(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = (self.regs[regs.1 as usize] + self.regs[regs.2 as usize]) % 4294967295;
    }
    fn multiply(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = (self.regs[regs.1 as usize] * self.regs[regs.2 as usize]) % 4294967295;
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
        let segment: Vec<u64> = vec![0 as u64; self.regs[(regs.2) as usize] as usize];
        let unused_key = (0..u32::MAX).into_iter().find(|key| !self.addr_space.contains_key(key)).unwrap();
        self.addr_space.insert(unused_key, segment);
        self.regs[regs.1 as usize] = unused_key;
    }
    fn unmap_seg(&mut self, regs: (u32, u32, u32)){
        self.addr_space.remove(&self.regs[regs.2 as usize]);
    }
    fn load_prog(&mut self, regs: (u32, u32, u32)){
        let new_seg = self.addr_space.get(&self.regs[regs.1 as usize]).unwrap();
        self.addr_space.insert(0 as u32, new_seg.to_vec());
        self.prog_cntr = self.regs[regs.2 as usize];
    }
    fn load_val(&mut self, reg: u32, val: u32){
        self.regs[reg as usize] = val;
    }
}

impl IoDevice for MchState {
    fn input(&mut self, regs: (u32, u32, u32)){
        match stdin().bytes().next() {
            Some(value) => {
                self.regs[regs.2 as usize] = value.unwrap() as u32;
            }
            None => self.regs[regs.2 as usize] = u32::MAX,
        }
    }
    fn output(&mut self, regs: (u32, u32, u32)){
        print!("{}", (self.regs[regs.2 as usize] as u8) as char);
    }
}

#[cfg(test)]
mod tests {

    use bitpack::bitpack::getu;
    use bitpack::bitpack::newu;
    use crate::mch_state::MchState;
    use crate::mch_state::UmOperations;
    use std::collections::HashMap;

    #[test]
    fn cond_move_test() {
        let r: [u32; 8] = [5, 0, 1, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u64>> = HashMap::new();
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
        let mut mem: HashMap<u32, Vec<u64>> = HashMap::new();

        let test_seg: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
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
        let mut mem: HashMap<u32, Vec<u64>> = HashMap::new();

        let test_seg: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        mem.insert(1 as u32, test_seg);

        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.seg_store(regs);
        assert!(machine.addr_space.get(&(1 as u32)).unwrap()[1] == 6 as u64);
    }
    #[test]
    fn add_test() {
        let r: [u32; 8] = [5, 1, 1, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u64>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.add(regs);
        assert!(machine.regs[0] == 2 as u32);
    }
    #[test]
    fn multiply_test() {
        let r: [u32; 8] = [5, 2, 2, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u64>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.multiply(regs);
        assert!(machine.regs[0] == 4 as u32);
    }
    #[test]
    fn division_test() {
        let r: [u32; 8] = [5, 2, 2, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u64>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.divide(regs);
        assert!(machine.regs[0] == 1 as u32);
    }
    #[test]
    fn nand_test() {
        let r: [u32; 8] = [5, u32::MAX, u32::MAX, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u64>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.nand(regs);
        assert!(machine.regs[0] == 0 as u32);
    }
    #[test]
    fn map_seg_test() {
        let r: [u32; 8] = [5, 0, 10, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u64>> = HashMap::new();
        let test_seg: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
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
        let mut mem: HashMap<u32, Vec<u64>> = HashMap::new();
        let test_seg: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
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
        let mut mem: HashMap<u32, Vec<u64>> = HashMap::new();
        let test_seg: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        mem.insert(0 as u32, test_seg);
        let test_seg2: Vec<u64> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        mem.insert(1 as u32, test_seg2);
        let test_seg3: Vec<u64> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.load_prog(regs);
        
        let x = machine.addr_space.get(&(0 as u32)).unwrap();
        for i in 0..test_seg3.len() {
            //assert!(x[i] == test_seg3[i]);
            println!("{0} {1}", x[i], test_seg3[i]);
        }
        println!("{}", machine.prog_cntr);
        assert!(machine.prog_cntr == 4 as u32);
    }
    #[test]
    fn load_val_test() {
        let r: [u32; 8] = [5, 0, 1, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u64>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let reg: u32 = 0;
        let value: u32 = 7;
        machine.load_val(reg, value);
        assert!(machine.regs[0] == value);
    }
}