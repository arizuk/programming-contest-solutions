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

fn find_right(bit: &BIT<usize>, n:usize, cur:usize, num: usize) -> usize {
    let mut ok = n+1;
    let mut ng = cur;
    while ok != ng {
        let mid = (ok + ng) / 2;
        if bit.range_sum(cur, mid+1) >= num {
            ok = mid;
        } else {
            ng = mid + 1;
        }
    }
    ok
}

fn find_left(bit: &BIT<usize>, n:usize, cur:usize, num: usize) -> usize {
    let cur_num = bit.range_sum(1, cur+1);
    if num >= cur_num {
        return 0;
    }

    let mut ok = cur;
    let mut ng = 1;
    while ok != ng {
        let mid = (ok + ng) / 2;
        if bit.range_sum(1, mid+1) >= (cur_num-num) {
            ok = mid;
        } else {
            ng = mid + 1;
        }
    }
    ok
}

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
    bit.add(pos[n]+1, 1);

    let mut ans = 0 as usize;
    for i in (1..n).rev() {
        let bit_pos = pos[i] + 1;
        let mut l1 = find_left(&bit, n, bit_pos, 1);
        let l2 = find_left(&bit, n, bit_pos, 0);
        let r1 = find_right(&bit, n, bit_pos, 1);

        if l1 == 1 && l2 == 1 {
            l1 = 0;
        }
        let r2 = find_right(&bit, n, bit_pos, 2);

        let v1 = (l2-l1) * (r1 - bit_pos) * i;
        let v2 = (r2-r1) * (bit_pos-l2) * i;

        ans += v1 + v2;
        debug!(i, l1, l2, bit_pos, r1, r2, v1, v2, ans);

        bit.add(bit_pos, 1);
    }
    puts!("{}\n", ans);
}
