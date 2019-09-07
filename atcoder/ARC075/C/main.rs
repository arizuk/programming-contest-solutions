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

pub fn lower_bound<T: Ord>(a: &Vec<T>, x: &T) -> usize {
    use std::cmp::Ordering;
    let mut l = 0;
    let mut r = a.len();
    while l != r {
        let m = l + (r - l) / 2;
        match a[m].cmp(x) {
            Ordering::Less => l = m + 1,
            Ordering::Equal | Ordering::Greater => r = m,
        }
    }
    l
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
      k: i64,
      aa: [i64; n],
    }

    let mut bb = vec![];
    let mut temp = 0;
    for i in 0..n {
        let a = aa[i];
        temp += a - k;
        bb.push(temp);
    }

    let mut comp = bb.clone();
    comp.sort();
    comp.dedup();

    let mut bit = BIT::new(comp.len());
    for i in 0..n {
        let b = bb[i];
        let idx = comp.binary_search(&b).unwrap();
        bit.add(idx+1, 1i64);
    }

    let mut ans = 0i64;
    let mut threshold = 0;
    for i in 0..n {
        let left = lower_bound(&comp, &threshold);
        ans += bit.range_sum(left+1, comp.len()+1);
        threshold += aa[i] - k;

        let b = bb[i];
        let idx = comp.binary_search(&b).unwrap();
        bit.add(idx+1, -1);
    }
    puts!("{}\n", ans);
}
