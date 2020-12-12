use std::collections::HashMap;
use std::io::{stdin, Read};

type BagChildren = Vec<(String, usize)>;

#[derive(Debug)]
struct Bag {
    color: String,
    children: BagChildren,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut map: HashMap<String, BagChildren> = HashMap::new();

    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .into_iter()
        .for_each(|record| {
            let line = parse_line(record);
            map.insert(line.color, line.children);
        });

    let mut ans = Vec::new();

    for k in map.keys() {
        if contains_bag(&map, "shiny gold", k) {
            ans.push(k.clone());
        }
    }

    println!("answer = {:?}", ans);

    println!("ans = {}", ans.len());
}

pub fn contains_bag(dag: &HashMap<String, BagChildren>, pattern: &str, src: &str) -> bool {
    let this = dag.get(src).unwrap();
    this.iter()
        .any(|(color, _)| color == pattern || contains_bag(dag, pattern, color))
}

fn parse_line(record: &str) -> Bag {
    let mut res = record
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim_end_matches(|x| !char::is_alphanumeric(x)).trim())
        .map(|x| x.replace("bags", "").replace("bag", ""));

    let first = res.next().unwrap();
    let mut first = first.split("contain").map(|x| x.trim());

    let src = first.next().unwrap().to_owned();

    if record.contains("contain no other bags") {
        return Bag {
            color: src,
            children: vec![],
        };
    }

    let mut children = vec![];

    let (ccount, cname) = parse_child(&first.next().unwrap());

    children.push((cname, ccount));

    for r in res {
        let (ccount, cname) = parse_child(&r);

        children.push((cname, ccount));
    }

    Bag {
        color: src,
        children,
    }
}

fn parse_child(a: &str) -> (usize, String) {
    let t = a
        .chars()
        .filter(|x| char::is_alphabetic(*x) | char::is_ascii_whitespace(x))
        .collect::<String>();

    let s = a
        .chars()
        .filter(|x| char::is_numeric(*x))
        .collect::<String>();

    let s = s.trim().parse::<usize>().unwrap();
    let t = t.trim().to_owned();

    (s, t)
}
