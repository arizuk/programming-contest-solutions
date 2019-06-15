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

static mut MOD: u64 = 1;

type Matrix = Vec<Vec<u64>>;

// Matrix Exponentiation
fn mat_mul(a: &Matrix, b: &Matrix) -> Matrix {
    assert!(a[0].len() == b.len());

    let h = a.len();
    let w = b[0].len();

    let mut ret = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            for k in 0..b.len() {
                unsafe {
                    ret[i][j] += (a[i][k] * b[k][j]) % MOD;
                    ret[i][j] %= MOD;
                }
            }
        }
    }
    ret
}

fn mat_pow(a: &Matrix, mut n: u64) -> Matrix {
    let mut a = a.clone();
    let mut b = vec![vec![0; a.len()]; a.len()];
    for i in 0..a.len() {
        b[i][i] = 1;
    }
    while n>0 {
        if n&1 > 0 {
            b = mat_mul(&b, &a);
        }
        a = mat_mul(&a, &a);
        n /= 2;
    }
    b
}

pub fn mod_pow(b: u64, p: u64) -> u64 {
    unsafe {
        if p == 0 {
            return 1;
        }
        let mut ret = mod_pow(b * b % MOD, p / 2) % MOD;
        if p % 2 == 1 {
            ret = ret * b % MOD;
        }
        ret
    }
}



#[doc = " [l, r)"]
pub fn binary_search_by<F>(mut l: u64, mut r: u64, f: &F) -> Option<u64>
where
    F: Fn(u64) -> bool,
{
    assert!(l <= r);
    let r_bound = r;
    while r != l {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    if r == r_bound {
        None
    } else {
        Some(r)
    }
}

fn number_of_digit(mut n: u64) -> usize {
    let mut num = 0;
    while n>0 {
        num += 1;
        n/= 10;
    }
    num
}

fn main() {
    input!{
      l: u64,
      a: u64,
      b: u64,
      m: u64,
    }
    unsafe {
        MOD = m;
    }
    const D: usize = 18;

    let sd = number_of_digit(a);
    let mut first_idx = vec![None; D+1];
    let mut acm = vec![l; D+1];
    acm[0] = 0;

    for d in sd..D+1 {
        let f = |mid| {
            let v = a + mid*b;
            return number_of_digit(v) >= d
        };
        if let Some(idx) = binary_search_by(0, l, &f) {
            acm[d-1] = idx;
            if number_of_digit(a + b*idx) == d {
                first_idx[d] = Some(idx);
            }
        }
    }
    let mut ans = 0;
    for d in 1..D+1 {
        if first_idx[d].is_none() {
            continue;
        }
        let idx = first_idx[d].unwrap();
        let first = (a + idx*b) % m;
        let td = 10u64.pow(d as u32) % m;
        let v = vec![
            vec![td,0,0],
            vec![1,1,0],
            vec![0,b,1],
        ];
        let fv = vec![
            vec![0, first, 1],
        ];

        let cd = acm[d] - acm[d-1];
        let power = mat_pow(&v, cd);
        let ret = mat_mul(&fv, &power);
        let x = ret[0][0];
        ans *= mod_pow(10, cd * d as u64);
        ans %= m;
        ans += x;
        ans %= m;
    }
    println!("{}", ans);
}
