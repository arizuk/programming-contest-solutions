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
      n: u64,
      i: u64,
      aa: [u64; n],
    }

    // lk種類までok
    let mut lk = 1;
    for large_k in 2..n+1 {
        let mut k = 0;
        let mut d = 1;
        while d < large_k {
            k += 1;
            d *= 2;
        }
        if k * n > i*8 {
            break;
        } else {
            lk = large_k;
        }
    }
    let mut bb = aa.clone();
    bb.sort();
    bb.dedup();
    let mut cnt = vec![0; bb.len()];
    for a in aa {
        let idx = bb.binary_search(&a).unwrap();
        cnt[idx] += 1;
    }

    let mut acm = vec![0; bb.len()+1];
    for i in 0..bb.len() {
        acm[i+1] = acm[i] + cnt[i];
    }

    let m = bb.len();
    let lk = lk as usize;
    if lk >= m {
        return puts!("{}\n", 0);
    }

    debug!(lk, cnt);

    let mut ans = n;
    let rem = m - lk;
    for l in 0..rem+1 {
        // 左からl種類消す
        let mut temp = acm[l];

        // 右からr種類
        let r = m+l-rem;

        temp += acm[m] - acm[r];

        debug!(l, r, temp);
        ans = min(ans, temp);
    }
    puts!("{}\n", ans);
}
