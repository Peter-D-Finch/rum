use std::collections::HashMap;
use bitpack::bitpack::getu;
use bitpack::bitpack::newu;
use std::io::stdin;
use std::io::Read;
use std::io::Write;

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
        //println!("test");
        let inst: u64 = (self.addr_space.get(&(0 as u32)).unwrap()[self.prog_cntr as usize]);
        let mut opcode = getu(inst, 4, 28);
        let mut regs = (getu(inst, 3, 6) as u32, getu(inst, 3, 3) as u32, getu(inst, 3, 0) as u32);
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
            13 => println!("load val: {0} {1} {2}", regs.0, regs.1, regs.2),
            _ => println!("default")
        }*/
        self.prog_cntr = self.prog_cntr + 1;
        if self.prog_cntr as usize == (self.addr_space.get(&(0 as u32)).unwrap().len() - 1) {
            self.halt();
        }
        else {
            self.exec_cycle();
        }
    }
    fn new(init_seg: Vec<u64>) {
        let mut r: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut pc: u32 = 0;
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
        let mut segment: Vec<u64> = vec![0 as u64; self.regs[(regs.2) as usize] as usize];
        let unused_key = (0..u32::MAX).into_iter().find(|key| !self.addr_space.contains_key(key)).unwrap();
        self.addr_space.insert(unused_key, segment);
    }
    fn unmap_seg(&mut self, regs: (u32, u32, u32)){
        self.addr_space.remove(&self.regs[regs.2 as usize]);
    }
    fn load_prog(&mut self, regs: (u32, u32, u32)){
        let new_seg = self.addr_space.get(&self.regs[regs.1 as usize]).unwrap();
        self.addr_space.insert(0 as u32, new_seg.to_vec());
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
            None => self.regs[regs.2 as usize] = !0 as u32,
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

    #[test]
    fn segment_test() {
        let temp_word: u32 = 1287363297;
        for i in 0..temp_word.to_be_bytes().len() {
            print!("{:08b} ", temp_word.to_be_bytes()[i]);
        }
        print!("\n");
        let mut temp_word2: u64 = temp_word as u64;
        temp_word2 = newu(temp_word2, 4, 0, 15).unwrap();
        for i in 0..temp_word2.to_be_bytes().len() {
            print!("{:08b} ", temp_word2.to_be_bytes()[i]);
        }
        print!("\n\n");
        let mut opcode = getu(temp_word2, 4, 0);
        let mut regs = (getu(temp_word2, 3, 22) as u32, getu(temp_word2, 3, 25) as u32, getu(temp_word2, 3, 28) as u32);
        for i in 0..opcode.to_be_bytes().len() {
            print!("{:08b} ", opcode.to_be_bytes()[i]);
        }
        print!("\n");
        for i in 0..(regs.0).to_be_bytes().len() {
            print!("{:08b} ", (regs.0).to_be_bytes()[i]);
        }
        print!("\n");
        for i in 0..(regs.1).to_be_bytes().len() {
            print!("{:08b} ", (regs.1).to_be_bytes()[i]);
        }
        print!("\n");
        for i in 0..(regs.2 as u32).to_be_bytes().len() {
            print!("{:08b} ", (regs.2).to_be_bytes()[i]);
        }
        print!("\n");
        assert!(1 == 2);
    }
}