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
    match (key, map.get(key)) {
        ("byr", Some(v)) => {
            let byr = v.parse::<i32>().unwrap();
            byr >= 1920 && byr <= 2002
        }
        ("iyr", Some(v)) => {
            let iyr = v.parse::<i32>().unwrap();
            iyr >= 2010 && iyr <= 2020
        }
        ("eyr", Some(v)) => {
            let eyr = v.parse::<i32>().unwrap();
            eyr >= 2020 && eyr <= 2030
        }
        ("hgt", Some(v)) => {
            if v.contains("cm") {
                let v = v.trim_end_matches(char::is_alphabetic);
                let v = v.parse::<i32>().unwrap();
                v >= 150 && v <= 193
            } else if v.contains("in") {
                let v = v.trim_end_matches(char::is_alphabetic);
                let v = v.parse::<i32>().unwrap();
                v >= 59 && v <= 76
            } else {
                false
            }
        }
        ("hcl", Some(v)) => {
            v.starts_with('#')
                && v.chars()
                    .skip(1)
                    .filter(|x| match x {
                        '0'..='9' => true,
                        'a'..='f' => true,
                        _ => false,
                    })
                    .count()
                    == 6
        }
        ("ecl", Some(v)) => match v.as_ref() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        ("pid", Some(v)) => v.len() == 9 && v.chars().filter(|&x| char::is_numeric(x)).count() == 9,
        ("cid", _) => true,
        (_, None) => false,
        (_, _) => unreachable!(),
    }
}
