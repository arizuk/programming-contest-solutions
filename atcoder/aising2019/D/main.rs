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

// [l, r)
fn bs(mut l: usize, mut r: usize, f: &Fn(usize) -> bool) -> usize {
    while r-l > 1 {
        let mid = (r+l)/2;
        if f(mid) {
            l = mid;
        } else {
            r = mid;
        }
    }
    l
}

fn main() {
    input!{
      n: usize,
      q: usize,
      aa: [i64; n],
      xs: [i64; q],
    }

    let mut s = vec![0; n+2];

    {
        let mut s0 = vec![0; n+1];
        for i in 0..n {
            s0[i+1] = s0[i] + aa[i];
        }

        let mut s1 = vec![0; n+2];
        for i in 0..n+2 {
            if i+2 <= n+1 {
                s1[i+2] = s1[i] + aa[i];
            }
        }

        for i in 1..(n/2+2) {
            let mut sum = s0[n] - s0[n-i];
            if 2*i < n {
                sum += s1[n+1-2*i];
            }
            s[i] = sum;
        }
    }

    for &x in xs.iter() {
        let f = |mut m: usize| -> bool {
            let mut k = n-m;
            if m == 0 {
                return true;
            };
            if m >= n-1 {
                return false;
            }

            if k%2 == 1 {
                m += 1;
                k = n-m;
            }

            let li = m-1;
            let ri = m+k/2-1;
            let l = (aa[li] - x).abs();
            let r = (aa[ri] - x).abs();
            // debug!(n, m, k, li, ri, l, r);
            if l > r { true } else { false }
        };

        let l = bs(0, n, &f);
        println!("{}", s[(n-l)/2 + (n-l)%2]);
    }
}
