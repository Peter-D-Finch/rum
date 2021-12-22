use std::env;

mod bitpack;
mod mch_state;
mod io_device;

use crate::bitpack::newu;
use crate::mch_state::UmFunctions;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = args.iter().nth(1).unwrap();
  let num_bytes = std::fs::metadata(filename).unwrap().len();
  assert!(num_bytes % 4 == 0);
  let num_words = num_bytes / 4;
  
  let bytes = std::fs::read(filename).unwrap();
  /*for i in 0..bytes.len() {
    for j in 0..(bytes[i]).to_be_bytes().len() {
      print!("{:08b} ", (bytes[i]).to_be_bytes()[j]);
    }
    print!("\n");
  }*/
  let mut words: Vec<u32> = Vec::new();
  for i in 0..num_words {
    //print!("[{}]", i);
    let mut temp_word: u32 = 0;
    temp_word = newu(temp_word as u64, 8, 0, bytes[(4*i+3) as usize] as u64).unwrap() as u32;
    temp_word = newu(temp_word as u64, 8, 8, bytes[((4*i)+2) as usize] as u64).unwrap() as u32;
    temp_word = newu(temp_word as u64, 8, 16, bytes[((4*i)+1) as usize] as u64).unwrap() as u32;
    temp_word = newu(temp_word as u64, 8, 24, bytes[((4*i)) as usize] as u64).unwrap() as u32;
    words.push(temp_word);
    /*for i in 0..(temp_word).to_be_bytes().len() {
      print!("{:08b} ",(temp_word).to_be_bytes()[i]);
    }
    print!("\n");*/
  }

  println!("\n{}", num_words);
  //mch_state::MchState::new(words);
}