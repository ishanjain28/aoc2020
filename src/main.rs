use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let questions = input
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|set| {
            let mut q = [false; 26];

            set.chars()
                .filter(|&x| char::is_alphabetic(x))
                .for_each(|y| {
                    q[(y as u8 - 97) as usize] = true;
                });

            q.iter().fold(0, |a, &x| if x { a + 1 } else { a })
        })
        .sum::<i32>();

    println!("questions = {:?}", questions);
}
