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
type I = usize;

fn main() {
    input!{
      n: usize,
      w: usize,
      wvs: [(usize, usize); n],
    }
    let w1 = wvs[0].0;
    let mut tbl = vec![vec![]; 4];
    for &(w, v) in wvs.iter() {
        tbl[w-w1].push(v);
    }
    for i in 0..4 {
        tbl[i].sort();
        tbl[i].reverse();
    }

    let mut ans = 0;
    for i in 0..tbl[0].len()+1 {
        for j in 0..tbl[1].len()+1 {
            for k in 0..tbl[2].len()+1 {
                for l in 0..tbl[3].len()+1 {
                    let cur_w = w1*i + (w1+1)*j + (w1+2)*k + (w1+3)*l;
                    if cur_w > w {
                        continue;
                    }
                    let mut v: usize = 0;
                    v += (&tbl[0][0..i]).iter().map(|&v| v).sum();
                    v += (&tbl[1][0..j]).iter().map(|&v| v).sum();
                    v += (&tbl[2][0..k]).iter().map(|&v| v).sum();
                    v += (&tbl[3][0..l]).iter().map(|&v| v).sum();
                    ans = max(ans, v);
                }
            }
        }
    }
    println!("{}", ans);
}
