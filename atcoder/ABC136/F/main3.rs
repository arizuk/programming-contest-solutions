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

pub mod ds {
    use std::ops::{AddAssign, Sub};
    #[derive(Debug)]
    pub struct BIT<T> {
        data: Vec<T>,
    }
    impl<T> BIT<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + Default,
    {
        pub fn new(size: usize) -> Self {
            let buf_size = size.next_power_of_two();
            BIT {
                data: vec![T::default(); buf_size + 1],
            }
        }
        #[doc = " [l, r) l,r: 1-indexed"]
        pub fn range_sum(&self, l: usize, r: usize) -> T {
            self.sum(r - 1) - self.sum(l - 1)
        }
        #[doc = " i: 1-indexed but returns 0 if i=0 is given."]
        pub fn sum(&self, i: usize) -> T {
            let mut i = i as i64;
            let mut ret = T::default();
            while i > 0 {
                ret += self.data[i as usize];
                i -= i & -i;
            }
            ret
        }
        #[doc = " i: 1-indexed"]
        pub fn add(&mut self, i: usize, value: T) {
            assert!(i > 0);
            let n = self.data.len() as i64;
            let mut i = i as i64;
            while i <= n - 1 {
                self.data[i as usize] += value;
                i += i & -i;
            }
        }
    }
}

use ds::BIT;


const MOD: usize = 998244353;

pub fn mod_pow(b: usize, p: usize) -> usize {
    if p == 0 {
        return 1;
    }
    let mut ret = mod_pow(b * b % MOD, p / 2) % MOD;
    if p % 2 == 1 {
        ret = ret * b % MOD;
    }
    ret
}
pub fn inv(a: usize) -> usize {
    mod_pow(a, MOD - 2)
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      mut xys: [(i64,i64); n],
    }
    let mut ys: Vec<_> = xys.iter().map(|v|v.1).collect();
    ys.sort();
    ys.dedup();
    xys.sort_by_key(|v|v.0);

    let m = ys.len();
    let mut bit = BIT::new(m);

    let mut points = vec![vec![0; 4]; n];
    for i in 0..n {
        let (_, y) = xys[i];
        let idx = ys.binary_search(&y).unwrap() + 1;

        let v0 = bit.range_sum(idx+1, m+1);
        let v2 = bit.range_sum(1, idx);

        points[i][0] = v0;
        points[i][2] = v2;

        bit.add(idx, 1);
    }

    for i in 0..n {
        let (_, y) = xys[i];
        let idx = ys.binary_search(&y).unwrap() + 1;

        let v0 = points[i][0];
        let v2 = points[i][2];

        let v1 = bit.range_sum(idx+1, m+1) - v0;
        let v3 = bit.range_sum(1, idx) - v2;

        points[i][1] = v1;
        points[i][3] = v3;
    }

    let mut ans = 0;
    for i in 0..n {
        for a in 0..2 {
            for b in 0..2 {
                for c in 0..2 {
                    for d in 0..2 {
                        let k0 = points[i][0];
                        let k1 = points[i][1];
                        let k2 = points[i][2];
                        let k3 = points[i][3];

                        let mut temp = 1;
                        if a == 1 {
                            temp *= mod_pow(2, k0) - 1;
                            temp %= MOD;
                        }
                        if b == 1 {
                            temp *= mod_pow(2, k1) - 1;
                            temp %= MOD;
                        }
                        if c == 1 {
                            temp *= mod_pow(2, k2) - 1;
                            temp %= MOD;
                        }
                        if d == 1 {
                            temp *= mod_pow(2, k3) - 1;
                            temp %= MOD;
                        }

                        // 点Pを内部に含むときは、点Pを含まない部分集合の場合も加算する
                        if (a==1 && d==1) || (b==1 && c==1) {
                            temp *= 2;
                            temp %= MOD;
                        }
                        ans += temp;
                        ans %= MOD;
                    }
                }
            }
        }
    }
    puts!("{}\n", ans);
}
