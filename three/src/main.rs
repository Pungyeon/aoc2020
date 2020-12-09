
fn main() {
    println!("trees: {}",
            Map::new(std::fs::read_to_string("input.txt").unwrap())
                .count_trees(Slope{x: 3, y: 1}));
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Coordinate {
    NONE,
    TREE,
}

struct Map {
    map: Vec<Vec<Coordinate>>,
}

impl Map {
    fn new(input: String) -> Self {
        let mut m = Map{
            map: Vec::new(),
        };
        for line in input.lines() {
            let mut ml : Vec<Coordinate> = Vec::new();
            for c in line.chars() {
                if c == '#' {
                    ml.push(Coordinate::TREE);
                } else {
                    ml.push(Coordinate::NONE);
                }
            }
            m.map.push(ml);
        }
        m
    }

    fn count_trees(&self, slope: Slope) -> i64 {
        let mut trees = 0;
        let mut x = 0;
        let mut y = 0;
        loop {
            match self.get(x, y) {
                Some(c) => {
                    if c == &Coordinate::TREE {
                        trees += 1;
                    }
                },
                None => break,
            }
            if y == self.map.len() {
                break;
            }
            x += slope.x;
            if x > self.map[0].len() {
                x = x % self.map[0].len();
            }
            y += slope.y;
            if y > self.map.len() {
                break;
            }
        };
        trees
    }

    fn is_tree(&self, x: usize, y: usize) -> bool {
        false
    }

    fn get(&self, x: usize, y: usize) -> Option<&Coordinate> {
        self.map.get(y)?.get(x)
    }
}

struct Slope {
    x: usize,
    y: usize,
}

#[test]
fn test_parse_input() {
    let input =
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    let m = Map::new(input.to_string());
    assert_eq!(m.get(2, 0), Some(&Coordinate::TREE));
    assert_eq!(m.get(0, 0), Some(&Coordinate::NONE));
    assert_eq!(m.get(6, 2), Some(&Coordinate::TREE));
    assert_eq!(m.count_trees(Slope{x: 3, y: 1}), 7);
}