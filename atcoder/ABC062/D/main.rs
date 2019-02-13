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
use std::cmp::Ordering;

#[allow(unused_imports)]
use std::io::Write;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rev(i64);

impl Ord for Rev {
    fn cmp(&self, other: &Rev) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Rev {
    fn partial_cmp(&self, other: &Rev) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

fn main() {
    input!{
      n: usize,
      aa: [i64; 3*n],
    }

    let mut left_heap = BinaryHeap::new();
    let mut left_sum = 0;
    let mut l_sums = vec![0; 3*n];

    let mut right_heap = BinaryHeap::new();
    let mut right_sum = 0;
    let mut r_sums = vec![0; 3*n];

    for i in 0..n {
        left_heap.push(Rev(aa[i]));
        left_sum += aa[i];
        l_sums[i] = left_sum;
    }
    for i in (2*n..3*n).rev() {
        right_heap.push(aa[i]);
        right_sum += aa[i];
        r_sums[i] = right_sum;
    }

    for i in n..2*n {
        if let Some(Rev(min_value)) = left_heap.pop() {
            if min_value < aa[i] {
                left_heap.push(Rev(aa[i]));
                left_sum += aa[i] - min_value;
            } else {
                left_heap.push(Rev(min_value));
            }
        }
        l_sums[i] = left_sum;
    }

    for i in (n..2*n).rev() {
        if let Some(max_value) = right_heap.pop() {
            if max_value > aa[i] {
                right_heap.push(aa[i]);
                right_sum += aa[i] - max_value;
            } else {
                right_heap.push(max_value);
            }
        }
        r_sums[i] = right_sum;
    }
    debug!(n, aa);
    debug!(l_sums);
    debug!(r_sums);
    let mut ans = l_sums[n-1] - r_sums[n];
    for i in n..2*n+1 {
        ans = max(l_sums[i-1] - r_sums[i], ans);
    }
    println!("{}", ans);
}
