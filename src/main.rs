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
            let seat = seat.chars();
            let mut region: u16 = 0;

            for s in seat {
                region <<= 1;
                region |= match s {
                    'B' | 'R' => 1,
                    'F' | 'L' | _ => 0,
                };
            }

            (region >> 3) * 8 + (region & 0b111)
        })
        .collect::<HashSet<u16>>();

    let min = *filled_seats.iter().min().unwrap();
    let max = *filled_seats.iter().max().unwrap();

    for i in min..max {
        if !filled_seats.contains(&i) {
            println!("seat = {}", i);
            break;
        }
    }
}
