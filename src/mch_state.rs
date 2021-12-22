use std::collections::HashMap;
use std::collections::LinkedList;
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
        let mut num_ops = 0;
        loop {
            let inst: u32 = self.addr_space.get(&(0 as u32)).unwrap()[self.prog_cntr as usize];
            let opcode = getu(inst as u64, 4, 28) as u32;
            let regs = (getu(inst as u64, 3, 6) as u32, getu(inst as u64, 3, 3) as u32, getu(inst as u64, 3, 0) as u32);
            // ----------------------------------- DEBUGGING ------------------------------------
            /*
            /*if opcode == 10 {
                println!("Should be outputting");
            }*/
            
            let opcode_string: String;
            match opcode {
                0 => opcode_string = "CM".to_string(),
                1 => opcode_string = "SL".to_string(),
                2 => opcode_string = "SS".to_string(),
                3 => opcode_string = "AD".to_string(),
                4 => opcode_string = "ML".to_string(),
                5 => opcode_string = "DV".to_string(),
                6 => opcode_string = "NN".to_string(),
                7 => opcode_string = "HL".to_string(),
                8 => opcode_string = "MS".to_string(),
                9 => opcode_string = "US".to_string(),
                10 => opcode_string = "OP".to_string(),
                11 => opcode_string = "IP".to_string(),
                12 => opcode_string = "LP".to_string(),
                13 => opcode_string = "LV".to_string(),
                _ => opcode_string = "FAIL".to_string()
            }
            print!("[OP #{0}][OPCODE {1}][OPREG {2} {3} {4}][PROG CNTR #{5}]: ", num_ops, opcode_string, regs.0, regs.1, regs.2, self.prog_cntr);
            */
            // ----------------------------------------------------------------------------------
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
            num_ops = num_ops + 1;
            //print!("\n");
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
            //print!("TRUE: reg {0} = reg {1} = {2}", regs.0, regs.1 as usize, self.regs[regs.1 as usize]);
            self.regs[regs.0 as usize] = self.regs[regs.1 as usize];
        }
    }
    fn seg_load(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = self.addr_space.get(&self.regs[regs.1 as usize]).unwrap()[self.regs[regs.2 as usize] as usize];
        //print!("reg {0} = m[{1}][{2}] = {3}", regs.0, self.regs[regs.1 as usize], self.regs[regs.2 as usize], self.regs[regs.0 as usize]);
    }
    fn seg_store(&mut self, regs: (u32, u32, u32)){
        let mut temp_vec: Vec<u32> = self.addr_space.get(&self.regs[regs.0 as usize]).unwrap().to_vec();
        temp_vec[self.regs[regs.1 as usize] as usize] = self.regs[regs.2 as usize];
        self.addr_space.insert(self.regs[regs.0 as usize], temp_vec);
        //print!("m[{0}][{1}] = reg {2} = {3}", self.regs[regs.0 as usize], self.regs[regs.1 as usize], regs.2, self.regs[regs.2 as usize]);
    }
    fn add(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = ((self.regs[regs.1 as usize] as i64 + self.regs[regs.2 as usize] as i64) % 4294967296) as u32;
        //print!("reg {0} = reg {1} + reg {2} = {3}", regs.0, regs.1 as usize, regs.2 as usize, self.regs[regs.0 as usize]);
    }
    fn multiply(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = (((self.regs[regs.1 as usize] as u64 * self.regs[regs.2 as usize] as u64) << 32) >> 32) as u32;
    }
    fn divide(&mut self, regs: (u32, u32, u32)){
        let result = (self.regs[regs.1 as usize] as i64) / (self.regs[regs.2 as usize] as i64);
        self.regs[regs.0 as usize] = result as u32;
    }
    fn nand(&mut self, regs: (u32, u32, u32)){
        self.regs[regs.0 as usize] = !(self.regs[regs.1 as usize] & self.regs[regs.2 as usize]);
        //print!("reg {0} = !({1} & {2}) = {3}", regs.0, self.regs[regs.1 as usize], self.regs[regs.2 as usize], self.regs[regs.0 as usize]);
    }
    fn halt(&mut self){
        //println!("\n\nSYSTEM HALTING...");
        std::process::exit(0x0000);
    }
    fn map_seg(&mut self, regs: (u32, u32, u32)){
        let segment: Vec<u32> = vec![0 as u32; self.regs[regs.2 as usize] as usize];
        let mut random: LinkedList<Vec<i32>> = LinkedList::new();
        random.push_back(vec![2, 3]);
        let mut unused_key: u32 = ((((((random.back().unwrap() as *const Vec<i32>) as u64) << 48) >> 48) as u32) * (((((random.back().unwrap() as *const Vec<i32>) as u64) << 48) >> 48) as u32)) % u32::MAX;
        let mut cntr = 0;
        while self.addr_space.contains_key(&unused_key) {
            let temp = vec![2, 3];
            random.push_back(temp);
            unused_key = ((((((random.back().unwrap() as *const Vec<i32>) as u64) << 47) >> 47) as u32) + (cntr*(((((random.back().unwrap() as *const Vec<i32>) as u64) << 56) >> 56) as u32))) % u32::MAX;
            cntr = cntr + 1;
        }
        self.addr_space.insert(unused_key, segment);
        self.regs[regs.1 as usize] = unused_key;
    }
    fn unmap_seg(&mut self, regs: (u32, u32, u32)){
        self.addr_space.remove(&self.regs[regs.2 as usize]);
    }
    fn load_prog(&mut self, regs: (u32, u32, u32)){
        //print!("goto m[{0}] line {1}", self.regs[regs.1 as usize], self.regs[regs.2 as usize]);
        if self.regs[regs.1 as usize] != 0 {
            let new_seg = self.addr_space.get(&self.regs[regs.1 as usize]).unwrap().to_vec();
            self.addr_space.remove(&(0 as u32));
            self.addr_space.insert(0 as u32, new_seg);
        }
        self.prog_cntr = self.regs[regs.2 as usize];
    }
    fn load_val(&mut self, reg: u32, val: u32){
        //print!("reg {0} = {1}", reg, val);
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
        let test_seg: Vec<u32> = vec![0, 1, 2, 16, 4, 5, 6, 7, 8, 9];
        mem.insert(1 as u32, test_seg);
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.seg_load(regs);
        assert!(machine.regs[regs.0 as usize] == 16 as u32);
    }
    #[test]
    fn seg_store_test() {
        let r: [u32; 8] = [5, 0, 10, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let test_seg: Vec<u32> = vec![0];
        mem.insert(0 as u32, test_seg);
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.map_seg(regs);

        regs = (1 as u32, 3 as u32, 2 as u32);
        machine.seg_store(regs);
        assert!(machine.addr_space.get(&machine.regs[1]).unwrap()[machine.regs[3] as usize] == 10 as u32);
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
        assert!(machine.regs[0] == ((machine.regs[3] as u64 + machine.regs[2] as u64) % 4294967296) as u32);
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
        assert!(machine.regs[0] == ((machine.regs[2] as u64 * machine.regs[3] as u64) % 4294967296) as u32);
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
        let test_seg: Vec<u32> = vec![0];
        mem.insert(0 as u32, test_seg);
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.map_seg(regs);
        assert!(machine.addr_space.get(&(machine.regs[regs.1 as usize] as u32)).unwrap()[0] == 0);
        assert!(machine.addr_space.get(&(machine.regs[regs.1 as usize] as u32)).unwrap()[9] == 0);
        assert!(machine.addr_space.get(&(machine.regs[regs.1 as usize] as u32)).unwrap().len() as u32 == machine.regs[regs.2 as usize]);
    }
    #[test]
    fn unmap_seg_test() {
        let r: [u32; 8] = [5, 0, 10, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mut mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let test_seg: Vec<u32> = vec![0];
        mem.insert(0 as u32, test_seg);
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let mut regs = (0 as u32, 1 as u32, 2 as u32);
        machine.map_seg(regs);
        assert!(machine.addr_space.get(&(machine.regs[regs.1 as usize] as u32)).unwrap()[0] == 0);
        assert!(machine.addr_space.get(&(machine.regs[regs.1 as usize] as u32)).unwrap()[9] == 0);
        assert!(machine.addr_space.get(&(machine.regs[regs.1 as usize] as u32)).unwrap().len() as u32 == machine.regs[regs.2 as usize]);
        let mut regs = (0 as u32, 1 as u32, 1 as u32);
        machine.unmap_seg(regs);
        let x = machine.addr_space.get(&(machine.regs[regs.1 as usize] as u32));
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
        let r: [u32; 8] = [5, 0, 0, 0, 0, 0, 0, 0];
        let pc: u32 = 0;
        let mem: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut machine: MchState = MchState { regs: r, prog_cntr: pc, addr_space: mem };

        let reg: u32 = 0;
        let value: u32 = 70000000;
        machine.load_val(reg, value);
        assert!(machine.regs[0] == value);
    }
}