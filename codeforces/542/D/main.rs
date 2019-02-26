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

fn dist(start_station: usize, station: usize, n: usize) -> usize {
    let mut tmp = station;
    if tmp < start_station {
        tmp += n;
    }
    (tmp-start_station) as usize
}

fn main() {
    input!{
      n: usize,
      m: usize,
      abs: [(usize, usize); m]
    }
    let mut tbl = vec![vec![]; n];

    for i in 0..m {
        let (a, b) = abs[i];
        tbl[a-1].push(b-1);
    }

    let mut dist_tbl = vec![0; n];
    for i in 0..n {
        if tbl[i].len() > 0 {
            let mut cur_dist = n * (tbl[i].len() - 1);
            // cur_dist += dist(start_station, i, n);
            cur_dist += tbl[i].iter().map(|v| dist(i, *v, n)).min().unwrap();
            dist_tbl[i] = cur_dist;
        }
    }

    for start_station in 0..n {
        let mut ans = 0;
        for i in 0..n {
            if tbl[i].len() > 0 {
                let cur_dist = dist_tbl[i] + dist(start_station, i, n);
                ans = max(ans, cur_dist);
            }
        }
        print!("{} ", ans);
    }
    println!();
}
