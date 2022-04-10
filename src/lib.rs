mod randomizer;

use std::{collections::HashMap, process};

use rand::prelude::*;
use randomizer::*;

pub fn run(config: Config){
    let map = generate_map(config.key);
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