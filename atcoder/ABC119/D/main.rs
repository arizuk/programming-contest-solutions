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
      a: i64,
      b: i64,
      q: usize,
      mut ss: [i64; a],
      mut ts: [i64; b],
      xs: [i64; q],
    }
    const INF: i64 = 1 << 50;
    ss.push(INF);
    ss.push(-INF);
    ss.sort();
    ts.push(INF);
    ts.push(-INF);
    ts.sort();

    for i in 0..q {
        let x = xs[i];
        let s = ss.binary_search(&x);
        let t = ts.binary_search(&x);
        let s = match s {
            Err(v) => v,
            _ => unreachable!()
        };
        let t = match t {
            Err(v) => v,
            _ => unreachable!()
        };

        let mut ans = INF;

        let d = max(ss[s], ts[t]) - x;
        ans = min(d, ans);

        let d = x-min(ss[s-1], ts[t-1]);
        ans = min(d, ans);

        let d = (ss[s]-x)*2 + x-ts[t-1];
        ans = min(ans, d);

        let d = (ss[s]-x) + (x-ts[t-1])*2;
        ans = min(ans, d);

        let d = (ts[t]-x)*2 + (x-ss[s-1]);
        ans = min(ans, d);

        let d = (ts[t]-x) + (x-ss[s-1])*2;
        ans = min(ans, d);

        println!("{}", ans);
    }
}
