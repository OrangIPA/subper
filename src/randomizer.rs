use rand::prelude::*;
use std::collections::HashMap;

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

pub fn inverse_map(seed: u64) -> HashMap<char, char> {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut result: HashMap<char, char> = HashMap::new();
    let mut input_vec: Vec<u8> = (0..26).collect();

    for i in 0..26u8 {
        let rand_index: usize = rng.gen_range(0..input_vec.len() as usize);
        let rand = input_vec.get(rand_index).unwrap();
        result.insert((*rand + 'a' as u8) as char,(i + 'a' as u8) as char);
        input_vec.swap_remove(rand_index);
    }
    result
}