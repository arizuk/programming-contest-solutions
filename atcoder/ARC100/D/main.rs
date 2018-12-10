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

const INF: usize = std::usize::MAX;

fn binaryserach(mut l: usize, mut r: usize, f: &Fn(usize) -> bool) -> usize {
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m;
        }
    }
    r
}

fn main() {
    input!{
        n: usize,
        aa: [usize; n],
    }
    let mut s = vec![0; n+1];
    for i in 0..aa.len() {
        s[i+1] = aa[i] + s[i];
    }
    let mut ans = INF;
    for i in 2..s.len()-2 {
        let l0 = binaryserach(1, i, &|m| {
            let b = s[m];
            let c = s[i] - s[m];
            return b >= c;
        });

        let r0 = binaryserach(i, s.len()-1, &|m| {
            let b = s[m] - s[i];
            let c = s[s.len()-1] - s[m];
            return b >= c;
        });

        let li_l = cmp::max(1 as isize, l0 as isize - 4) as usize;
        let li_u = cmp::min(i as isize, l0 as isize + 4) as usize;
        let ri_l = cmp::max(i as isize, r0 as isize -4) as usize;
        let ri_u = cmp::min(s.len(), r0 + 4);

        // println!("l0={} r0={}", l0, r0);
        for j in li_l..li_u {
            for k in ri_l..ri_u {
                let p = s[j] - s[0];
                let q = s[i] - s[j];
                let r = s[k] - s[i];
                let n = s[s.len()-1] - s[k];
                let max = cmp::max(cmp::max(p, q), cmp::max(r, n));
                let min = cmp::min(cmp::min(p, q), cmp::min(r, n));
                ans = cmp::min(max - min, ans);
                // println!("j={} i={} k={} {:?} | {:?} | {:?} | {:?} {}", j, i, k, &s[0..j+1], &s[j+1..i+1], &s[i+1..k+1], &s[k+1..], max-min);
            }
        }
    }
    println!("{}", ans);
}
