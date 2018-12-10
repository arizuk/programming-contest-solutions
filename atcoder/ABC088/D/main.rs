#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};
use std::collections::VecDeque;

fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin };

    let h: usize = sc.read();
    let w: usize = sc.read();

    let mut ss = vec![];
    for _ in 0..h {
        ss.push(sc.chars());
    }

    let mut costs = vec![vec![-1i64; w]; h];
    costs[0][0] = 0;

    let mut q: VecDeque<(i64, i64, i64)> = VecDeque::new();
    q.push_back((0, 0, 1));
    while q.len() >= 1 {
        let (x, y, c) = q.pop_front().unwrap();
        let n_cost = c + 1;
        for &(dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= h as i64 { continue };
            if ny < 0 || ny >= w as i64 { continue };

            let nx = nx as usize;
            let ny = ny as usize;
            if ss[nx][ny] == '#' { continue };

            if n_cost >= costs[nx][ny] && costs[nx][ny] != -1 {
                continue;
            }
            costs[nx][ny] = n_cost;
            q.push_back((nx as i64, ny as i64, n_cost));
        }
    }

    let cost = costs[h-1][w-1];
    let mut cnt = 0;
    for row in ss {
        for c in row {
            if c == '.' {
                cnt += 1;
            }
        }
    }
    if cost == -1 {
        println!("{}", -1);
    } else {
        println!("{}", cnt-cost);
    }
}


pub struct Scanner<R> {
    reader: R,
}
impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}