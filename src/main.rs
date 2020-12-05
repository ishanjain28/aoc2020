use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let max_seat_id: usize = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|seat| {
            let seat = seat.as_bytes();
            let (region, columns) = seat.split_at(7);

            let mut lower = 0;
            let mut upper = 127;

            for r in region {
                match r {
                    b'F' => {
                        let mid = (lower + upper) / 2;
                        upper = mid;
                    }
                    b'B' => {
                        let mid = (lower + upper + 1) / 2;
                        lower = mid;
                    }
                    _ => unreachable!(),
                }
            }

            let region = lower;
            lower = 0;
            upper = 7;

            for col in columns {
                match col {
                    b'L' => {
                        let mid = (lower + upper) / 2;
                        upper = mid;
                    }
                    b'R' => {
                        let mid = (lower + upper + 1) / 2;
                        lower = mid;
                    }
                    _ => unreachable!(),
                }
            }

            let col = lower;

            region * 8 + col
        })
        .max()
        .unwrap();

    println!("{:?}", max_seat_id);
}
