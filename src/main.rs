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

            let mut limits = limits.split('-');
            let lowerlimit = limits.next().unwrap().parse::<usize>().unwrap();
            let upperlimit = limits.next().unwrap().parse::<usize>().unwrap();

            (lowerlimit, upperlimit, chars, pwd)
        })
        .filter(|(lowerlimit, upperlimit, chars, pwd)| {
            let count = &pwd.chars().filter(|x| x == chars).count();
            count >= lowerlimit && count <= upperlimit
        })
        .count();

    println!("{}", count);
}
