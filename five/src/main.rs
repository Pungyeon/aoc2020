use std::borrow::Borrow;

fn main() {
    match std::fs::read_to_string("input.txt") {
        Ok(r) => {
            let ids = r.lines().map(|line| seat_id(line.borrow())).max();
            println!("result: {}", ids.unwrap_or(0));
        },
        Err(e) => println!("err: {}", e.to_string()),
    }
}

fn seat_id(input: &str) -> i64 {
    let (row, col) = partition(input);

    (row * 8) + col
}

fn partition(input: &str) -> (i64, i64) {
    let (row, n) = custom_partition(input, 0, 127, 'F', 'B');
    let (col, _) = custom_partition(&input[n+1..], 0, 7, 'L', 'R');
    (row, col)
}

fn custom_partition(input: &str, mut lower: i64, mut upper: i64, lb: char, ub: char) -> (i64, usize) {
    for (i, c) in input.chars().enumerate() {
        let mid = (upper - lower) / 2;
        if c == ub {
            lower = lower + mid + 1;
        } else if c == lb {
            upper = upper - mid - 1;
        }
        if upper == lower {
            return (upper, i)
        }
    }
    (upper, input.len()-1)
}

#[test]
fn read_boarding_pass() {
    let (row, col) = partition("FBFBBFFRLR");
    assert_eq!(row, 44);
    assert_eq!(col, 5);
    assert_eq!(357, seat_id("FBFBBFFRLR"));

    let (row, col) = partition("BFFFBBFRRR");
    assert_eq!(row, 70);
    assert_eq!(col, 7);

    let (row, col) = partition("FFFBBBFRRR");
    assert_eq!(row, 14);
    assert_eq!(col, 7);

    let (row, col) = partition("BBFFBBFRLL");
    assert_eq!(row, 102);
    assert_eq!(col, 4);
}