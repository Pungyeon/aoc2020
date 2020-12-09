
fn main() {
    let map = Map::new(std::fs::read_to_string("input.txt").unwrap());
    println!("trees: {}", map.count_trees(Slope{x: 3, y: 1}));
    println!("slope_products: {}", map.slope_products(
        vec![
            Slope{ x: 1, y: 1},
            Slope{ x: 3, y: 1},
            Slope{ x: 5, y: 1},
            Slope{ x: 7, y: 1},
            Slope{ x: 1, y: 2},
        ]
    ));
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

    fn slope_products(&self, slopes: Vec<Slope>) -> i64 {
        let values : Vec<i64> = slopes.iter()
            .cloned()
            .map(|slope| self.count_trees(slope)).collect();

        let mut total = 1;
        for value in values {
            total *= value;
        }
        total
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
            if x >= self.map[0].len() {
                x = x % self.map[0].len();
            }
            y += slope.y;
            if y >= self.map.len() {
                break;
            }
        };
        trees
    }

    fn get(&self, x: usize, y: usize) -> Option<&Coordinate> {
        self.map.get(y)?.get(x)
    }
}

#[derive(Clone)]
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

    // PART TWO
    assert_eq!(m.count_trees(Slope{ x: 1, y: 1 }), 2);
    assert_eq!(m.count_trees(Slope{ x: 5, y: 1 }), 3);
    assert_eq!(m.count_trees(Slope{ x: 7, y: 1 }), 4);
    assert_eq!(m.count_trees(Slope{ x: 1, y: 2 }), 2);

    assert_eq!(m.slope_products(
        vec![
        Slope{ x: 1, y: 1},
        Slope{ x: 3, y: 1},
        Slope{ x: 5, y: 1},
        Slope{ x: 7, y: 1},
        Slope{ x: 1, y: 2},
    ]), 336);

}