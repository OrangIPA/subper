use std::{collections::HashMap, process};

use rand::prelude::*;

pub fn run(config: Config){
    let map = generate_map(config.key);
}

pub fn gen_rand_u8(seed: u64, input_char: char) -> u8 {
    let mut rng = StdRng::seed_from_u64(seed);
    let random_integer: u8 = rng.gen_range(0..26);
    ((input_char as u8 + random_integer) % 26) + 'a' as u8
}

pub fn generate_map(seed: u64) -> HashMap<char, char> {
    let mut rng = StdRng::seed_from_u64(seed);
    
    let mut result: HashMap<char, char> = HashMap::new();
    let mut input_vec: Vec<u8> = (0..26).collect();
    
    for i in 0..26u8 {
        let rand_index: usize = rng.gen_range(0..input_vec.len() as usize);
        let rand = input_vec.get(rand_index).unwrap();
        result.insert((i + 'a' as u8) as char, (*rand + 'a' as u8) as char);
        input_vec.swap_remove(rand_index);
    }
    result
}

pub struct Config {
    pub key: u64,
    pub text: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config , &str>{
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        
        let key:u64 = args[1].clone().parse().unwrap_or_else(|_err|{
            eprintln!("Key must be an unsigned 64 bit integer");
            process::exit(1)
        });
        let text = args[2].clone();

        Ok(Config {
            key,
            text
        })
    }
}

#[cfg(test)]
mod tests{
    use crate::*;

    #[test]
    pub fn print_random(){
        let out_vektor = generate_map(12);
        // let i = out_vektor.iter();
    
        for val in out_vektor{
            println!("{}", val.0);
        }
    
        // assert_eq!(26, outVektor.len());
    }
}