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
use std::cmp::{min, max};

#[allow(unused_imports)]
use std::io::Write;

const INF: i64 = 1 << 50;

fn dfs(ls: &[i64], i: usize, a: i64, b: i64, c: i64, abc: &[i64]) -> i64 {
    if i == ls.len() {
        if min(a, min(b, c)) == 0 {
            return INF;
        }
        return (abc[0]-a).abs() + (abc[1]-b).abs() + (abc[2]-c).abs() - 30;
    }
    let ret1 = dfs(ls, i+1, a, b, c, abc);
    let ret2 = dfs(ls, i+1, a+ls[i], b, c, abc) + 10;
    let ret3 = dfs(ls, i+1, a, b+ls[i], c, abc) + 10;
    let ret4 = dfs(ls, i+1, a, b, c+ls[i], abc) + 10;
    min(ret1, min(min(ret2, ret3), ret4))
}

fn main() {
    input!{
      n: usize,
      a: i64,
      b: i64,
      c: i64,
      ls: [i64; n]
    }
    let ans = dfs(&ls, 0, 0, 0, 0, &[a, b, c]);
    println!("{}", ans);
}
