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

    fn is_valid_part_two(&self, password: String) -> bool {
        let left = password.as_bytes()[self.lower-1] as char == self.letter;
        let right = password.as_bytes()[self.upper-1] as char == self.letter;

        (left || right) && (left != right)
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
        self.index += 3;

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
        panic!("could not find the thing");
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

fn find_valid_part_two() -> usize {
    std::fs::read_to_string("input.txt").unwrap().lines().filter(|line| {
        is_valid_part_two(line)
    }).count()
}

fn is_valid_part_two(line: &str) -> bool {
    let mut parser = Parser::new(line);
    let constraint = parser.constraint();
    let password = line[parser.index..].to_string();

    constraint.is_valid_part_two(password)
}

fn main() {
    println!("Valid Passwords: {}", find_valid());
    println!("Valid Passwords: {}", find_valid_part_two());
}

#[test]
fn test_part_two() {
    assert_eq!(is_valid_part_two("17-18 z: zzzzzzzzzzzzzzzzjz"), true);
    assert_eq!(is_valid_part_two("2-9 c: ccccccccc"), false);
    assert_eq!(is_valid_part_two("1-3 a: abcde"), true);
}