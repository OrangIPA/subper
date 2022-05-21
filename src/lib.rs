mod randomizer;

use std::process;

use randomizer::*;

pub fn run(mut config: Config) {
    config.text = config.text.to_ascii_lowercase();
    let map = match config.mode {
        Mode::Encrypt => generate_map(config.key),
        Mode::Decrypt => inverse_map(config.key),
    };
    
    let mut result: String = String::new();

    for chars in config.text.chars() {
        if let Some(x) = map.get(&chars) {
            result = format!("{}{}", result, x);
        } else {
            result = format!("{}{}", result, chars);
            continue;
        }
    }
    println!("{}", result);
}

pub enum Mode {
    Encrypt,
    Decrypt,
}

pub struct Config {
    pub key: u64,
    pub text: String,
    pub mode: Mode,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let key: u64 = args[1].clone().parse().unwrap_or_else(|_err| {
            eprintln!("Key must be an unsigned 64 bit integer");
            process::exit(1)
        });
        let text = args[2].clone();

        let mode = match &args[3].to_lowercase()[0..] {
            "e" => Ok(Mode::Encrypt),
            "d" => Ok(Mode::Decrypt),
            _ => Err(())
        }.unwrap_or_else(|_err|{
            eprintln!("invalid mode");
            process::exit(1);
        });

        Ok(Config { key, text, mode })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn print_random() {
        let out_vektor = generate_map(12);
        // let i = out_vektor.iter();

        for val in out_vektor {
            println!("{}", val.0);
        }

        // assert_eq!(26, outVektor.len());
    }
}
