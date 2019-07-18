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
const MOD: u64 = 998244353;

fn get_digit(mut n: u64) -> u64 {
    let mut d = 0;
    while n>0 {
        n/=10;
        d += 1;
    }
    d
}

fn f(mut a: u64, mut k: u64, left: bool) -> u64 {
    assert!(k > 0);
    let mut ret = 0;

    let mut d;
    if left {
        d = 10;
        k -= 1;
    } else {
        d = 1;
    }
    while a > 0 {
        ret += a%10 * d;
        ret %= MOD;

        a /= 10;
        d *= 10;
        if k > 0 {
            d *= 10;
            k -= 1;
        }
        d %= MOD;
    }
    ret
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }


    input!{
      n: usize,
      aa: [u64; n],
    }
    let digits: Vec<_> = aa.iter().map(|&v| get_digit(v)).collect();
    // debug!(digits);
    let mut nums = vec![0; 12];
    for &d in digits.iter() {
        nums[d as usize] += 1;
    }

    // for i in 1..10 {
    //     let v1 = f(123, i, true);
    //     let v2 = f(123, i, false);
    //     debug!(i, v1, v2);
    // }
    let mut ans = 0;
    for a in aa {
        let mut a = a;
        for d in 1..12 {
            let v1 = f(a, d, true);
            let v2 = f(a, d, false);
            let num = nums[d as usize];

            ans += (v1 * num) % MOD;
            ans %= MOD;
            ans += (v2 * num) % MOD;
            ans %= MOD;
        }
    }
    puts!("{}", ans);
}
