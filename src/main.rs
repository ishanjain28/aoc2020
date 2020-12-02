use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let numbers: HashSet<u64> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .filter(|&x| x <= 2020)
        .collect();

    'outer: for (i, &num1) in numbers.iter().enumerate() {
        let diff = 2020 - num1;

        for (j, &num2) in numbers.iter().enumerate() {
            if i == j {
                continue;
            }
            if numbers.contains(&(diff - num2)) {
                println!("{}", num1 * num2 * (diff - num2));
                break 'outer;
            }
        }
    }
}
