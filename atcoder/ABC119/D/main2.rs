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
use std::cmp::Ordering;

#[doc = " Equivalent to std::lowerbound and std::upperbound in c++"]
pub trait BinarySearch<T> {
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

fn main() {
    input!{
      a: i64,
      b: i64,
      q: usize,
      mut ss: [i64; a],
      mut ts: [i64; b],
      xs: [i64; q],
    }
    ss.sort();
    ts.sort();

    for i in 0..q {
        let x = xs[i];
        let s1 = ss.lower_bound(&x) as i64;
        let t1 = ts.lower_bound(&x) as i64;
        let s2 = s1-1;
        let t2 = t1-1;
        let mut ans = 1 << 60;

        let get_s = |i| (ss[i as usize] - x).abs();
        let get_t = |i| (ts[i as usize] - x).abs();

        if s1 <= a-1 && t1 <= b-1 {
            ans = min(ans, max(get_s(s1), get_t(t1)));
        }

        if s2 >= 0 && t2 >= 0 {
            ans = min(ans, max(get_s(s2), get_t(t2)));
        }

        if s1 <= a-1 && t2 >= 0 {
            let d = get_s(s1)*2 + get_t(t2);
            ans = min(ans, d);

            let d = get_s(s1) + get_t(t2)*2;
            ans = min(ans, d);
        }

        if s2 >= 0 && t1 <= b-1 {
            let d = get_t(t1)*2 + get_s(s2);
            ans = min(ans, d);

            let d = get_t(t1) + get_s(s2)*2;
            ans = min(ans, d);
        }

        println!("{}", ans);
    }
}
