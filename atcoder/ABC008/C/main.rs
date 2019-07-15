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

// n1Ck1 * n2Ck2 / n3Ck3
fn ep(n1:usize, k1:usize, n2:usize, k2:usize, n3:usize, k3:usize) -> f64 {
    if k1 > n1 || k2 > n2 {
        return 0.0
    }

    let mut ans = 1.0;
    for i in 0..k1 {
        ans *= (n1-i) as f64;
        ans /= (k1-i) as f64;
    }
    for i in 0..k2 {
        ans *= (n2-i) as f64;
        ans /= (k2-i) as f64;
    }

    for i in 0..k3 {
        ans /= (n3-i) as f64;
        ans *= (k3-i) as f64;
    }
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      cs: [usize; n],
    }

    let mut inv = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if cs[i] % cs[j] == 0 {
                inv[i] += 1;
            }
        }
    }

    let mut ans = 0.0;
    for i in 0..n {
        let c = cs[i];
        let inv_num = inv[i];
        let ep = if inv_num % 2 == 1 {
            0.5
        } else {
            ((inv_num as f64 / 2.0) + 1.0) / (inv_num as f64 + 1.0)
        };
        ans += ep;
        // debug!(c, ep);
    }
    puts!("{}", ans);
}
