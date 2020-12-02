use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let numbers: HashSet<u64> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for &num in numbers.iter() {
        if numbers.contains(&(2020 - num)) {
            println!("{}", num * (2020 - num));
            break;
        }
    }
}
