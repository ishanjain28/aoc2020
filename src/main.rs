use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let count = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut x = x.split(':');
            let policy = x.next().unwrap().trim();
            let pwd = x.next().unwrap().trim();
            let chars = policy.chars().last().unwrap();

            let mut policy = policy.split_whitespace();
            let limits = policy.next().unwrap();

            let mut positions = limits.split('-');
            let lpos = positions.next().unwrap().parse::<usize>().unwrap();
            let rpos = positions.next().unwrap().parse::<usize>().unwrap();

            (lpos, rpos, chars, pwd)
        })
        .filter(|&(lpos, rpos, c, pwd)| {
            let pwd: Vec<char> = pwd.chars().collect();

            (pwd[lpos - 1] == c || pwd[rpos - 1] == c) && (pwd[lpos - 1] != pwd[rpos - 1])
        })
        .count();

    println!("{}", count);
}
