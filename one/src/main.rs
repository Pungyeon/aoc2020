
use std::fs;
use std::collections::{HashMap, HashSet};

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
            println!("{} {}", a, b);
            return Ok(a * b);
        }
        map.insert(a, true);
    }
    Err(ValueError::NoResult)
}

fn get_values() -> Result<i64, ValueError> {
    let mut values : Vec<i64> = Vec::new();
    for line in fs::read_to_string("input.txt")?.lines() {
        values.push(line.parse()?);
    }

    for i in 0..values.len() {
        let a = values.get(i).unwrap();
        for j in i..values.len() {
            let b = values.get(j).unwrap();
            for k in j..values.len() {
                let c = values.get(k).unwrap();
                if a + b + c == 2020 {
                    return Ok(a*b*c);
                }
            }
        }
    }

    Err(ValueError::NoResult)
}

fn main() {
    println!("{:?}", get_value()); // part one
    println!("{:?}", get_values()); // part two
}
