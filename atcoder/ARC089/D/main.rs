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
      k: usize,
      xycs: [(usize, usize, String); n]
    }

    let mut sxy: Vec<Vec<usize>> = vec![vec![0; k*2+1]; k*2+1];
    let mut xys = vec![vec![0; k*2]; k*2];

    for &(x, y, ref s) in xycs.iter() {
        let x = x%(2*k);
        let mut y = y%(2*k);
        if *s == 'W'.to_string() {
            y = (y+k)%(2*k)
        }
        xys[x][y] += 1;
    }
    for x in 0..2*k {
        for y in 0..2*k {
            sxy[x+1][y+1] = sxy[x+1][y] + sxy[x][y+1] + xys[x][y] - sxy[x][y];
        }
    }

    let sum = |x: usize, lx: usize, y: usize, ly: usize| -> usize {
        sxy[x+lx][y+ly] + sxy[x][y] - sxy[x+lx][y] - sxy[x][y+ly]
    };

    let mut ans = 0;
    for x in 0..k {
        for y in 0..k {
            // (x+k-1),(y+k-1)
            // (x+k-1),(y-1)
            // (x-1),(y+k-1)
            // (x-1),(y-1)
            let mut s = sum(x, k, y, k);
            s += sum(x+k, k-x, 0, y);
            s += sum(0, x, y+k, k-y);
            s += sum(x+k, k-x, y+k, k-y);
            s += sum(0, x, 0, y);
            ans = max(ans, max(s, n-s));
        }
    }
    println!("{}", ans);
}