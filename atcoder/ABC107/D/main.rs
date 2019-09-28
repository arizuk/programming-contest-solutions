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
    use std::cmp::{min, PartialOrd};
    use std::ops::{AddAssign, Sub};
    #[derive(Debug)]
    pub struct BIT<T> {
        size: usize,
        data: Vec<T>,
    }
    impl<T> BIT<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + PartialOrd + From<u16>,
    {
        pub fn new(size: usize) -> Self {
            let buf_size = size.next_power_of_two();
            BIT {
                size: size,
                data: vec![T::from(0u16); buf_size + 1],
            }
        }
        #[doc = " [l, r) l,r: 1-indexed"]
        pub fn sum_between(&self, l: usize, r: usize) -> T {
            self.sum(r - 1) - self.sum(l - 1)
        }
        #[doc = " i: 1-indexed but returns 0 if i=0 is given."]
        pub fn sum(&self, i: usize) -> T {
            let mut i = i as i64;
            let mut ret = T::from(0u16);
            while i > 0 {
                ret += self.data[i as usize];
                i -= i & -i;
            }
            ret
        }
        #[doc = " i: 1-indexed"]
        pub fn add(&mut self, i: usize, value: T) {
            assert!(i > 0 && i <= self.size);
            let n = self.data.len() as i64;
            let mut i = i as i64;
            while i <= n - 1 {
                self.data[i as usize] += value;
                i += i & -i;
            }
        }
        pub fn lower_bound(&self, mut value: T) -> usize {
            let zero = T::from(0u16);
            if value <= zero {
                return 0;
            }
            let n = self.data.len();
            let mut x = 0;
            let mut k = n - 1;
            while k > 0 {
                if x + k <= n - 1 && self.data[x + k] < value {
                    value = value - self.data[x + k];
                    x += k;
                }
                k /= 2;
            }
            x = min(x, self.size);
            x + 1
        }
    }
}
use ds::BIT;

fn is_ok(aa: &Vec<usize>, x: usize) -> bool {
    let n = aa.len();
    let offset = n as i64;
    let mut total = 0i64;
    let mut sum = 0;
    let mut bit = BIT::new(2*n+1);
    bit.add(offset as usize +1, 1);
    for i in 0..n {
        if aa[i] >= x {
            total += 1;
        } else {
            total -= 1;
        }

        let pos = (total+offset+1) as usize;
        sum += bit.sum(pos);
        bit.add(pos, 1u64);
    }
    let num = n*(n+1)/2;
    let ans = (num+1)/2 <= sum as usize;
    debug!(x, (num+1)/2, sum, ans);
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      aa: [usize; n],
    }
    let mut bb = aa.clone();
    bb.sort();

    let mut ng = n as i64;
    let mut ok = -1;
    while (ok-ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        debug!(mid);
        if is_ok(&aa, bb[mid as usize]) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    puts!("{}\n", bb[ok as usize]);
}
