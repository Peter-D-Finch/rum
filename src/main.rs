use std::env;
use bitpack::bitpack::newu;

mod mch_state;
use crate::mch_state::UmFunctions;

fn main() {
  let args: Vec<String> = env::args().collect();
  let argnum = args.len();
  assert!(argnum == 1);
  //let filename = args.iter().nth(2).unwrap();
  let num_bytes = std::fs::metadata("/csc/411/um/cat.um").unwrap().len();
  assert!(num_bytes % 4 == 0);
  let num_words = num_bytes / 4;
  
  let bytes = std::fs::read("/csc/411/um/cat.um").unwrap();
  /*for i in 0..bytes.len() {
    for j in 0..(bytes[i]).to_be_bytes().len() {
      print!("{:08b} ", (bytes[i]).to_be_bytes()[j]);
    }
    print!("\n");
  }*/
  let mut words: Vec<u64> = Vec::new();
  for i in 0..num_words {
    let mut temp_word: u64 = 0;
    temp_word = newu(temp_word, 8, 0, bytes[(4*i+3) as usize] as u64).unwrap();
    temp_word = newu(temp_word, 8, 8, bytes[((4*i)+2) as usize] as u64).unwrap();
    temp_word = newu(temp_word, 8, 16, bytes[((4*i)+1) as usize] as u64).unwrap();
    temp_word = newu(temp_word, 8, 24, bytes[((4*i)) as usize] as u64).unwrap();
    words.push(temp_word);
    /*for i in 0..(temp_word).to_be_bytes().len() {
      print!("{:08b} ", (temp_word).to_be_bytes()[i]);
    }
    print!("\n");*/
  }
  //print!("\n");
  //println!("{}", num_words);
  mch_state::MchState::new(words);
}