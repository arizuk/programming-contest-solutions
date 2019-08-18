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
use std::io::{stdout, stdin, BufWriter, Write};

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      xys: [(i64,i64); n],
    }

    let xys: Vec<_> = xys.iter().map(|&(x, y)| (x+y, x-y)).collect();
    let mut xs: Vec<_> = xys.iter().map(|v| v.0).collect();
    xs.sort();
    xs.dedup();
    let mut ys: Vec<_> = xys.iter().map(|v| v.1).collect();
    ys.sort();
    ys.dedup();

    let x1 = xs[0];
    let x2 = xs[xs.len()-1];
    let y1 = ys[0];
    let y2 = ys[ys.len()-1];
    let square_len = max(x2-x1, y2-y1);
    assert!(square_len%2 == 0);

    let min_x = xs[0];
    let max_x = xs[xs.len()-1];
    let min_y = ys[0];
    let max_y = ys[ys.len()-1];

    let possibles = [
        (min_x + square_len/2, min_y + square_len/2),
        (max_x - square_len/2, min_y + square_len/2),
        (max_x - square_len/2, max_y - square_len/2),
        (min_x + square_len/2, max_y - square_len/2),
    ];

    let rev_rotate = |x, y| ((x+y)/2, (x-y)/2);
    for &(px, py) in possibles.iter() {
        let p = rev_rotate(px, py);
        let mut dists: Vec<_> = xys.iter().map(|v| rev_rotate(v.0, v.1)).map(|v| (p.0-v.0).abs() + (p.1-v.1).abs()).collect();
        dists.sort();
        dists.dedup();
        if dists.len() == 1 {
            return puts!("{} {}\n", p.0, p.1);
        }
    }
    unreachable!();
}
