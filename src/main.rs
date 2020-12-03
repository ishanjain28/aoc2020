use std::io::{stdin, Read};

#[derive(Debug, PartialEq, Eq)]
enum GridType {
    Open,
    Tree,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let grid = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|x| {
            x.as_bytes()
                .into_iter()
                .map(|y| match y {
                    b'#' => GridType::Tree,
                    b'.' => GridType::Open,
                    _ => unreachable!(),
                })
                .collect::<Vec<GridType>>()
        })
        .collect::<Vec<Vec<GridType>>>();

    let m = grid.len();
    let n = grid[0].len();
    println!("m = {} n = {}", m, n);

    let a = calc_trees(&grid, m, n, 3, 1);
    let b = calc_trees(&grid, m, n, 1, 1);
    let c = calc_trees(&grid, m, n, 5, 1);
    let d = calc_trees(&grid, m, n, 7, 1);
    let e = calc_trees(&grid, m, n, 1, 2);

    println!("{} {} {} {} {}", a, b, c, d, e);

    println!("{}", a * b * c * d * e);
}

fn calc_trees(grid: &[Vec<GridType>], m: usize, n: usize, slope_x: usize, slope_y: usize) -> u32 {
    let mut sx = 0;
    let mut sy = 0;
    let mut treecount = if grid[0][0] == GridType::Open { 0 } else { 1 };

    while sx < m - 1 {
        sy = (sy + slope_x) % n;
        sx += slope_y;

        if grid[sx][sy] == GridType::Tree {
            treecount += 1;
        }
    }
    return treecount;
}
