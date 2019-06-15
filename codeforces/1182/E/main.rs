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

fn mul(a: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    assert!(a[0].len() == b.len());

    let h = a.len();
    let w = b[0].len();

    let mut ret = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            for k in 0..b.len() {
                ret[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    ret
}

fn pow(a: &Vec<Vec<u64>>, mut n: u64) -> Vec<Vec<u64>> {
    let mut a = a.clone();
    let mut b = vec![vec![0; a.len()]; a.len()];
    for i in 0..a.len() {
        b[i][i] = 1;
    }
    while n>0 {
        if n&1 > 0 {
            b = mul(&b, &a);
        }
        a = mul(&a, &a);
        n /= 2;
    }
    b
}

const MOD: u64 = 1e9 as u64 + 7;

pub fn mod_pow(b: u64, p: u64) -> u64 {
    if p == 0 {
        return 1;
    }
    let mut ret = mod_pow(b * b % MOD, p / 2) % MOD;
    if p % 2 == 1 {
        ret = ret * b % MOD;
    }
    ret
}


fn main() {
    input!{
      n: u64,
      f1: u64,
      f2: u64,
      f3: u64,
      c: u64,
    }

    let mut a = vec![
        vec![1,1,1],
        vec![1,0,0],
        vec![0,1,0],
    ];

    let v1 = vec![
        vec![0],
        vec![0],
        vec![1],
    ];
    let v2 = vec![
        vec![0],
        vec![1],
        vec![0],
    ];
    let v3 = vec![
        vec![1],
        vec![0],
        vec![0],
    ];

    let power = pow(&a, n-3);
    let p1 = mul(&power, &v1)[0][0];
    let p2 = mul(&power, &v2)[0][0];
    let p3 = mul(&power, &v3)[0][0];
    let mut ans = mod_pow(f1, p1) * mod_pow(f2, p2) * mod_pow(f3, p3);
    ans *= mod_pow(c, 6);
    println!("{}", ans);
}
