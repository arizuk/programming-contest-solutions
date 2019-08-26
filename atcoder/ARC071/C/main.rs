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
      s: chars,
      t: chars,
      q: usize,
      abcds: [(usize1, usize1, usize1,usize1); q],
    }
    let n = s.len();
    let m = t.len();

    let mut dp1 = vec![vec![0i64; 2]; n+1];
    let mut dp2 = vec![vec![0i64; 2]; m+1];

    for i in 0..n {
        let idx = if s[i] == 'A' { 0 } else { 1 };
        dp1[i+1][idx] += dp1[i][idx] + 1;
        dp1[i+1][1-idx] += dp1[i][1-idx];
    }

    for i in 0..m {
        let idx = if t[i] == 'A' { 0 } else { 1 };
        dp2[i+1][idx] += dp2[i][idx] + 1;
        dp2[i+1][1-idx] += dp2[i][1-idx];
    }


    for i in 0..q {
        let (a, b, c, d) = abcds[i];

        let sa = dp1[b+1][0] - dp1[a][0];
        let sb = dp1[b+1][1] - dp1[a][1];

        let ta = dp2[d+1][0] - dp2[c][0];
        let tb = dp2[d+1][1] - dp2[c][1];


        let sn = sa + sb * 2;
        let tn = ta + tb * 2;

        if (tn - sn) % 3 == 0 {
            puts!("{}\n", "YES");
        } else {
            puts!("{}\n", "NO");
        }
    }
}
