use std::error::Error;

struct Constraint {
    lower: usize,
    upper: usize,
    letter: char,
}

impl Constraint {
    fn is_valid(&self, password: String) -> bool {
        let seen : usize = password.chars()
            .filter(|x| x == &self.letter)
            .count();

        seen >= self.lower && seen <= self.upper
    }
}

struct Parser<'a> {
    line: &'a str,
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(line: &'a str) -> Self {
        Parser{
            line,
            index: 0,
        }
    }

    fn constraint(&mut self) -> Constraint {
        let mut constraint = Constraint {
            lower: 0,
            upper: 0,
            letter: ' ',
        };
        constraint.lower = self.read_until('-').parse().unwrap();
        self.index += 1;

        constraint.upper = self.read_until(' ').parse().unwrap();
        self.index += 1;

        constraint.letter = self.line.as_bytes()[self.index] as char;
        self.index += 2;

        constraint
    }

    fn read_until(&mut self, until: char) -> String {
        let start = self.index;
        while self.index < self.line.len() {
            if self.line.as_bytes()[self.index] as char == until {
                return self.line[start..self.index].to_string();
            }
            self.index += 1;
        }
        "".to_string()
    }
}

fn find_valid() -> usize {
    std::fs::read_to_string("input.txt").unwrap().lines().filter(|line| {
        let mut parser = Parser::new(line);
        let constraint = parser.constraint();
        let password = line[parser.index..].to_string();

        constraint.is_valid(password)
    }).count()
}


fn main() {
    println!("Valid Passwords: {}", find_valid());
}
