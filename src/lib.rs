use rand::prelude::*;

pub fn run(config: Config){
    
}

pub fn gen_rand_u8(seed: u64, inputChar: char) -> u8 {
    let mut rng = StdRng::seed_from_u64(seed);
    let randomInteger: u8 = rng.gen_range(0..26);
    ((inputChar as u8 + randomInteger) % 26) + 'a' as u8
}

pub fn rand_from_vec(seed: u64, mut inputVec: Vec<u8>) -> Vec<u8>{
    let mut rng = StdRng::seed_from_u64(seed);
    
    let mut result: Vec<u8> = vec![];
    
    for i in (0..inputVec.len()){
        let mut randIndex: usize = rng.gen_range(0..inputVec.len() as usize);
        let rand = inputVec.get(randIndex).unwrap();
        result.push(*rand);
        inputVec.swap_remove(randIndex);
    }
    result
}

pub struct Config {
    pub key: String,
    pub text: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config , &str>{
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        
        let key = args[1].clone();
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
    fn print_random(){
        let vektor: Vec<u8> = (0..26).collect();
        let outVektor: Vec<u8> = rand_from_vec(12, vektor);
        let mut i = 0;
        for val in outVektor{
            println!("{} -> {}",(i + 'a' as u8) as char, (val + 'a' as u8) as char);
            i += 1;
        }
        // assert_eq!(26, outVektor.len());
    }
}