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
    use super::*;
    use std::cmp::PartialOrd;
    use std::ops::{AddAssign, Sub};
    #[derive(Debug)]
    pub struct BIT<T> {
        size: usize,
        data: Vec<T>,
    }
    impl<T> BIT<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + Default + PartialOrd + std::fmt::Debug,
    {
        pub fn new(size: usize) -> Self {
            let buf_size = size.next_power_of_two();
            BIT {
                size: size,
                data: vec![T::default(); buf_size + 1],
            }
        }
        #[doc = " [l, r) l,r: 1-indexed"]
        pub fn sum_between(&self, l: usize, r: usize) -> T {
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
        pub fn lower_bound(&self, value: T) -> usize {
            let mut r = self.size + 1;
            let mut l = 1;
            while r != l {
                let mid = (r + l) / 2;
                if self.sum(mid) >= value {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            r
        }
    }
}
use ds::BIT;

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      ps: [usize; n],
    }

    let mut pos = vec![0; n+1];
    for i in 0..n {
        let p = ps[i];
        pos[p] = i;
    }

    let mut bit = BIT::new(n);
    let mut ans = 0 as usize;
    for i in (1..n+1).rev() {
        let bit_pos = pos[i] + 1;
        bit.add(bit_pos, 1);
        let cur = bit.sum(bit_pos) as i64;

        let get_index = |v| {
            if v <= 0 {
                0
            } else {
                bit.lower_bound(v as usize)
            }
        };
        let l1 = get_index(cur-2);
        let l2 = get_index(cur-1);
        let r1 = get_index(cur+1);
        let r2 = get_index(cur+2);

        ans += (l2-l1) * (r1-bit_pos) * i;
        ans += (r2-r1) * (bit_pos - l2) * i;
    }
    puts!("{}\n", ans);
}
