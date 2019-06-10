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



fn main() {
    input!{
      n: usize,
      x: usize,
      mut blus: [(usize, usize, usize); n]
    }
    let mut imps: Vec<(usize, usize)> = blus.iter().map(|v| v.2 * (x as usize - v.0) + v.1 * v.0).enumerate().collect();
    imps.sort_by_key(|v| v.1);
    imps.reverse();
    let mut score1: usize = blus.iter().map(|v| v.0 * v.1).sum();
    let mut score2: usize = 0;
    let mut cur = 0;
    let mut ans = 0;

    while score2 < score1 {
        let (idx, _) =  imps[cur];
        let &(b, l, u) = &blus[idx];
        // debug!(score1, score2, idx, b, l, u);

        let (t, c) = if score2 + l*b >= score1 {
            ((score1 - score2 + l - 1) / l, l)
        } else {
            ((score1 + (b*u - b*l) - score2 + u - 1) / u, u)
        };
        let t = min(t, x);
        ans += t;
        score2 += t * c;
        score1 += b*c - b*l;

        // debug!(t, score1, score2, c);
        cur += 1;
    }
    println!("{}", ans);
}
