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
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

fn solve(s: &mut [usize], fs: &Vec<Vec<usize>>, ps: &Vec<Vec<i64>>, i: usize) -> i64 {
    if i == s.len() {
        let mut p = 0;
        for n in 0..fs.len() {
            let mut cnt = 0;
            for j in 0..fs[n].len() {
                if fs[n][j] == 1 && s[j] == 1 { cnt+= 1 }
            }
            p += ps[n][cnt];
        }
        return p;
    }

    s[i] = 0;
    if i == s.len()-1 && s.iter().fold(0, |sum, i| sum + i) == 0 {
        s[i] = 1;
    }
    let a1 = solve(s, fs, ps, i+1);
    s[i] = 1;
    let a2 = solve(s, fs, ps, i+1);
    return max(a1, a2)
}


fn main() {
    input!{
      n: usize,
      fs: [[usize; 10]; n],
      ps: [[i64; 11]; n],
    }

    let mut s = vec![0; 10];
    let ans = solve(&mut s, &fs, &ps, 0);
    println!("{}", ans);
}
