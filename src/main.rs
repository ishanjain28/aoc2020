use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let filled_seats = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|seat| {
            let mut region_lower = 0;
            let mut region_upper = 127;
            let mut col_lower = 0;
            let mut col_upper = 7;

            for s in seat.as_bytes() {
                match s {
                    b'F' => {
                        let mid = (region_lower + region_upper) / 2;
                        region_upper = mid;
                    }
                    b'B' => {
                        let mid = (region_lower + region_upper + 1) / 2;
                        region_lower = mid;
                    }
                    b'L' => {
                        let mid = (col_lower + col_upper) / 2;
                        col_upper = mid;
                    }
                    b'R' => {
                        let mid = (col_lower + col_upper + 1) / 2;
                        col_lower = mid;
                    }
                    _ => unreachable!(),
                }
            }
            let region = region_lower;
            let col = col_lower;

            region * 8 + col
        })
        .collect::<HashSet<usize>>();

    let min = *filled_seats.iter().min().unwrap();
    let max = *filled_seats.iter().max().unwrap();

    for i in min..max {
        if !filled_seats.contains(&i) {
            println!("seat = {}", i);
            break;
        }
    }
}
