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

#[allow(unused_imports)]
use std::cmp;

const INF: isize = std::isize::MAX;

fn calc(s: &[isize], l: usize, m: usize, r: usize) -> isize {
    let p = s[l];
    let q = s[m] - s[l];
    let rr = s[r] - s[m];
    let n = s[s.len()-1] - s[r];
    let max = cmp::max(cmp::max(p, q), cmp::max(rr, n));
    let min = cmp::min(cmp::min(p, q), cmp::min(rr, n));
    // println!("l={} i={} r={} {:?} | {:?} | {:?} | {:?} {}", l, m, r, &s[0..l+1], &s[l+1..m+1], &s[m+1..r+1], &s[r+1..], max-min);
    assert!(l > 0);
    assert!(l < m);
    assert!(m < r);
    assert!(r < s.len());
    max - min
}

fn main() {
    input!{
        n: usize,
        aa: [isize; n],
    }
    let mut s = vec![0; n+1];
    for i in 0..aa.len() {
        s[i+1] = aa[i] + s[i];
    }
    let mut f = vec![0; n+1];
    let mut g = vec![0; n+1];

    let mut ans = INF;
    for i in 2..n-1 {
        // f
        let mut j = f[i-1];
        let b = s[j];
        let c = s[i] - s[j];
        let mut dmin = (b - c).abs();
        while j < i-1 {
            let b = s[j+1];
            let c = s[i] - b;
            if dmin < (b - c).abs() { break; }
            dmin = (b - c).abs();
            j = j + 1;
        }
        f[i] = j;

        // g
        let mut j = cmp::max(g[i-1], i+1);
        let b = s[j] - s[i];
        let c = s[n] - s[j];
        let mut dmin = (b - c).abs();
        while j < n {
            let b = s[j+1] - s[i];
            let c = s[n] - s[j+1];
            if dmin < (b - c).abs() { break; }
            dmin = (b - c).abs();
            j = j + 1;
        }
        g[i] = j;
        ans = cmp::min(calc(&s, f[i], i, g[i]), ans);
    }
    println!("{}", ans);
}
