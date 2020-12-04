use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let passports: usize = input
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|x| x.replace('\n', " "))
        .map(|x| {
            let mut map = HashMap::new();

            x.split(' ').filter(|x| !x.is_empty()).for_each(|y| {
                let mut y = y.split(':');
                let key = y.next().unwrap();
                let value = y.next().unwrap();

                map.insert(key.to_owned(), value.to_owned());
            });

            map
        })
        .filter(|map| {
            validate(&map, "byr")
                && validate(&map, "iyr")
                && validate(&map, "eyr")
                && validate(&map, "hgt")
                && validate(&map, "hcl")
                && validate(&map, "ecl")
                && validate(&map, "pid")
        })
        .count();

    println!("{:?}", passports);
}

fn validate(map: &HashMap<String, String>, key: &str) -> bool {
    if let Some(v) = map.get(key) {
        !v.is_empty()
    } else {
        false
    }
}
