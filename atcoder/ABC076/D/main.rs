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
      ts: [f64; n],
      mut vs: [f64; n],
    }

    vs.push(0.0);
    let mut v2s = vec![0.0; n];
    for i in 0..n {
        let mut v0 = if i == 0 { 0.0 } else { v2s[i-1] };
        let t = ts[i];
        let v1 = vs[i];
        let mut v2 = v0 + t;
        v2 = v2.min(v1).min(vs[i+1]);

        if v0 > v2 && v0-v2 > t {
            v0 = v2 + t;
            v2s[i-1] = v0;
        }
        v2s[i] = v2;
    }

    // d = v0*t + t*t/2
    let mut v0 = 0.0;
    let mut ans = 0.0;
    for i in 0..n {
        let t = ts[i];
        let v1 = vs[i];
        let v2 = v2s[i];
        assert!(v1 >= v0);

        // debug!(t, v0, v1, v2);

        let mut t1 = (v2 + t - v0) / 2.0;
        t1 = t1.min(t);

        let t2 = t - t1;
        let nx_v = v0 + t1 - t2;

        // 左側
        let mut dist = v0 * t1 + t1*t1/2.0;

        // 右側
        dist += (v0 + t1) * t2 - t2*t2/2.0;

        // 超過した分
        if v1 <= v0 + t1 {
            let t3 = v0 + t1 - v1;
            dist += t3 * v1 * 2.0;
            dist -= (v1 * t3 + (t3*t3)/2.0) * 2.0;
        }
        // debug!(dist, v0, v1, v2, nx_v, t, t1, t2);

        v0 = v0 + t1 - t2;
        ans += dist;
    }
    puts!("{}\n", ans);
}
