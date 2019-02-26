#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{max, min};

#[allow(unused_imports)]
use std::io::Write;

fn dfs(map: &mut [Vec<char>], i: i64, j: i64, c: char) {
    map[i as usize][j as usize] = c;

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let n = map.len() as i64;
    for &(dx, dy) in dirs.iter() {
        let nx = dx + i;
        let ny = dy + j;
        if !(nx >= 0 && nx < n && ny >= 0 && ny < n) {
            continue;
        }
        if map[nx as usize][ny as usize] != '0' {
            continue;
        }
        dfs(map, nx, ny, c);
    }
}

fn find_smallest(map: &[Vec<char>], x: usize, y: usize) -> i64 {
    let n = map.len();
    let mut ans = 1 << 30;
    for i in 0..n {
        for j in 0..n {
            if map[i][j] == '3' {
                let d = (x as i64 - i as i64).pow(2) + (y as i64 - j as i64).pow(2);
                ans = min(ans, d);
            }
        }
    }
    ans
}

fn main() {
    input!{
      n: usize,
      mut r1: i64,
      mut c1: i64,
      mut r2: i64,
      mut c2: i64,
      mut map: [chars; n]
    }
    dfs(&mut map, r1 - 1, c1 - 1, '2');
    if map[r2 as usize - 1][c2 as usize - 1] == '2' {
        return println!("{}", 0);
    }
    dfs(&mut map, r2 - 1, c2 - 1, '3');

    let mut ans = 1 << 30;
    for i in 0..n {
        for j in 0..n {
            if map[i][j] == '2' {
                ans = min(ans, find_smallest(&map, i, j));
            }
        }
    }
    println!("{}", ans);
}
