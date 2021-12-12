use std::collections::HashMap;
use bitpack::bitpack::getu;
use bitpack::bitpack::newu;
use std::io::stdin;
use std::io::Read;
use std::io::Write;

struct MchState {
	regs: [u32; 8],
	prog_cntr: u32,
	addr_space: HashMap<u32, Vec<u64>>
}

struct Segment {
    words: Vec<u64>,
    len: usize
}

pub trait SegOperations {
    fn get_word(&mut self, idx: u64) -> u32;
    fn set_word(&mut self, idx: u64, value: u32);
    fn new(words: Vec<u32>) -> Self;
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

pub trait IoDevice {
    fn input(&mut self, regs: (u32, u32, u32));
    fn output(&mut self, regs: (u32, u32, u32));
}

 // 0 0 0 0 _ _ _ _ _ _ _  _  _  _  _  _  _  _  _  _  _  _  _  1  1  1  2  2  2  3  3  3 
 // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31

 // 0  0  0  0  _  _  _  _  _  _  _  _  _  _  _  _  _  _  _  _  _  _  _  1  1  1  2  2  2  3  3  3 
 // 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63

impl SegOperations for Segment {
    fn get_word(&mut self, idx: u64) -> u32 {
        let word: u32;
        let pckd_word = self.words[(idx / 2) as usize];
        match idx%2 {
            0 => word = (getu(pckd_word, 32, 0) << 32) as u32,
            1 => word = getu(pckd_word, 32, 31) as u32,
            _ => word = 5
        }
        return word;
    }
    fn set_word(&mut self, idx: u64, value: u32) {
        let pckd_word = self.words[(idx / 2) as usize];
        match idx%2 {
            0 => self.words[(idx / 2) as usize] = newu(pckd_word, 32, 0, value as u64).unwrap(),
            1 => self.words[(idx / 2) as usize] = newu(pckd_word, 32, 31, value as u64).unwrap(),
            _ => println!("Something is VERY wrong")
        }
    }
    fn new(words: Vec<u32>) -> Self {
        let mut pckd_words: Vec<u64> = Vec::new();
        if words.len()%2 == 0 {
            for i in 0..words.len()/2 {
                let mut temp_word: u64 = 0;
                temp_word = newu(temp_word, 32, 0, words[i * 2] as u64).unwrap();
                for i in 0..temp_word.to_be_bytes().len() {
                    print!("{:08b} ", temp_word.to_be_bytes()[i]);
                }
                print!("\n");
                temp_word = newu(temp_word, 32, 31, words[(i * 2) + 1] as u64).unwrap();
                for i in 0..temp_word.to_be_bytes().len() {
                    print!("{:08b} ", temp_word.to_be_bytes()[i]);
                }
                print!("\n");
                pckd_words.push(temp_word);
            }
        }
        else {
            for i in 0..(words.len()/2)+1 {
                let mut temp_word: u64 = 0;
                if i != words.len()/2 {
                    temp_word = newu(temp_word, 32, 0, words[i * 2] as u64).unwrap();
                    temp_word = newu(temp_word, 32, 31, words[(i * 2) + 1] as u64).unwrap();
                    pckd_words.push(temp_word);
                }
                else {
                    temp_word = newu(temp_word, 32, 0, words[i * 2] as u64).unwrap();
                    pckd_words.push(temp_word);
                }
            }
        }

        let new_seg = Segment {
            words: pckd_words,
            len: words.len()
        };
        return new_seg;
    }
}

impl UmFunctions for MchState {
    fn exec_cycle(&mut self) {
        let pckd_inst = self.addr_space.get(&(0 as u32)).unwrap()[self.prog_cntr as usize];
        let mut opcode = getu(pckd_inst, 4, 4);
        let mut regs = (getu(pckd_inst, 3, 25) as u32, getu(pckd_inst, 3, 28) as u32, getu(pckd_inst, 3, 31) as u32);
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
            13 => self.load_val(getu(pckd_inst, 3, 6) as u32, getu(pckd_inst, 25, 31) as u32),
            _ => println!("default")
        }
        self.prog_cntr = self.prog_cntr + 1;
        opcode = getu(pckd_inst, 4, 35);
        regs = (getu(pckd_inst, 3, 57) as u32, getu(pckd_inst, 3, 60) as u32, getu(pckd_inst, 3, 63) as u32);
        match opcode {
            0 => println!("cond move"),
            1 => println!("seg load"),
            2 => println!("seg store"),
            3 => println!("addition"),
            4 => println!("multiplication"),
            5 => println!("division"),
            6 => println!("bitwise nand"),
            7 => println!("halt"),
            8 => println!("map_seg"),
            9 => println!("unmap_seg"),
            10 => println!("output"),
            11 => println!("input"),
            12 => println!("load_prog"),
            13 => println!("load_val"),
            _ => println!("default")
        }
    }
    fn new(init_seg: Vec<u32>) {
        println!("TODO");
    }
}

impl UmOperations for MchState {
    fn cond_move(&mut self, regs: (u32, u32, u32)){
        if self.regs[regs.2 as usize] != (0 as u32) {
            self.regs[regs.0 as usize] = self.regs[regs.1 as usize];
        }
    }
    fn seg_load(&mut self, regs: (u32, u32, u32)){
        let pckd_inst = self.addr_space.get(&self.regs[regs.1 as usize]).unwrap()[(self.regs[regs.2 as usize]/2) as usize];
        let inst: u64;
        match self.regs[regs.2 as usize]%2 {
            0 => inst = getu(pckd_inst, 32, 31),
            1 => inst = getu(pckd_inst, 32, 63),
            _ => inst = newu(0 as u64, 4, 3, 7).unwrap()
        }
        self.regs[regs.0 as usize] = u32::try_from(inst).unwrap();
    }
    fn seg_store(&mut self, regs: (u32, u32, u32)){
        let pckd_inst = self.addr_space.get(&self.regs[regs.0 as usize]).unwrap()[(self.regs[regs.1 as usize]/2) as usize];
        let inst: u64;
        match self.regs[regs.2 as usize]%2 {
            0 => inst = newu(pckd_inst, 32, 31, self.regs[regs.2 as usize] as u64).unwrap(),
            1 => inst = newu(pckd_inst, 32, 63, self.regs[regs.2 as usize] as u64).unwrap(),
            _ => inst = 0
        }
        self.addr_space.get_mut(&self.regs[regs.0 as usize]).unwrap()[(self.regs[regs.1 as usize]/2) as usize] = inst;
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
        let mut segment: Vec<u64>;
        match self.regs[(regs.2) as usize] % 2 {
            0 => segment = vec![0 as u64; self.regs[(regs.2) as usize] as usize / 2],
            1 => segment = vec![0 as u64; (self.regs[(regs.2) as usize] as usize / 2) + 1],
            _ => println!("Something is VERY wrong.")
        }
        let mut segment: Vec<u64> = vec![0 as u64; self.regs[(regs.2/2) as usize] as usize];
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
        println!("{}", self.regs[regs.2 as usize]);
    }
}
#[cfg(test)]
mod tests {

    use crate::mch_state::Segment;
    use crate::mch_state::SegOperations;

    #[test]
    fn segment_test() {
        let words: Vec<u32> = vec![0, 1, 2, 3];
        let mut segment = Segment::new(words);
        println!("{0} {1} {2} {3}", segment.get_word(0 as u64), segment.get_word(1 as u64), segment.get_word(2 as u64), segment.get_word(3 as u64));
        assert!(segment.get_word(0 as u64)==0);
        println!("{0} {1} {2} {3}", segment.get_word(0 as u64), segment.get_word(1 as u64), segment.get_word(2 as u64), segment.get_word(3 as u64));
        segment.set_word(0 as u64, 1 as u32);
        println!("{0} {1} {2} {3}", segment.get_word(0 as u64), segment.get_word(1 as u64), segment.get_word(2 as u64), segment.get_word(3 as u64));
        assert!(segment.get_word(0 as u64)==1);
    }
}