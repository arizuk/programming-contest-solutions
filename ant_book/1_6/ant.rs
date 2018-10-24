use std::cmp;

fn solve(l: u32, xs: &[u32]) {
    let mut max = 0;
    let mut min = 0;

    for &x in xs.iter() {
        max = cmp::max(cmp::max(x, l - x), max);
        min = cmp::max(cmp::min(x, l - x), min);
    }
    println!("min={}, max={}", min, max);
}

fn main() {
    solve(10, &[2, 6, 7]);
}
