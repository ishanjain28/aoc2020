use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let questions = input
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|set| {
            let groups: Vec<&str> = set.split('\n').filter(|x| !x.is_empty()).collect();
            let mut q = vec![vec![false; groups.len()]; 26];

            for (i, group) in groups.into_iter().enumerate() {
                group.chars().for_each(|x| {
                    q[(x as u8 - 97) as usize][i] = true;
                });
            }

            q.into_iter().filter(|x| !x.is_empty()).fold(0, |a, x| {
                if x.into_iter().all(|y| y) {
                    a + 1
                } else {
                    a
                }
            })
        })
        .sum::<i32>();

    println!("questions = {:?}", questions);
}
