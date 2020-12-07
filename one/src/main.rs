
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum ValueError {
    IO(std::io::Error),
    Parse(std::num::ParseIntError),
    NoResult,
}

impl From<std::io::Error> for ValueError {
    fn from(err: std::io::Error) -> ValueError {
        ValueError::IO(err)
    }
}

impl From<std::num::ParseIntError> for ValueError {
    fn from(err: std::num::ParseIntError) -> ValueError {
        ValueError::Parse(err)
    }
}

fn get_value() -> Result<i64, ValueError> {
    let mut map: HashMap<i64, bool> = HashMap::new();
    for line in fs::read_to_string("input.txt")?.lines() {
        let a = line.parse()?;
        let b = 2020 - a;
        if map.get(&b).is_some() {
            return Ok(a * b);
        }
        map.insert(a, true);
    }
    Err(ValueError::NoResult)
}

fn main() {
    println!("{:?}", get_value());
}
