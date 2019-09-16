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
    use std::cmp::{min, max, PartialOrd, Ord};
    use std::ops::{AddAssign, Sub};
    #[derive(Debug)]
    pub struct BIT<T> {
        size: usize,
        data: Vec<T>,
    }
    impl<T> BIT<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + PartialOrd + Ord + From<usize>,
    {
        pub fn new(size: usize) -> Self {
            let buf_size = size.next_power_of_two();
            BIT {
                size: size,
                data: vec![T::from(0 as usize); buf_size + 1],
            }
        }
        #[doc = " i: 1-indexed but returns 0 if i=0 is given."]
        pub fn query(&self, i: usize) -> T {
            let mut i = i as i64;
            let mut ret = T::from(0 as usize);
            while i > 0 {
                ret = max(ret, self.data[i as usize]);
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
                self.data[i as usize] = max(self.data[i as usize], value);
                i += i & -i;
            }
        }
    }
}

fn solve1(w:usize, vws: &[(usize,usize)]) -> Vec<(usize, usize)> {
    let n = vws.len();
    let mut pairs = vec![];
    for bit in 0..1<<n {
        let mut v = 0;
        let mut ww = 0;
        for i in 0..n {
            if bit & (1 << i) > 0 {
                v += vws[i].0;
                ww += vws[i].1;
            }
        }
        if ww <= w {
            pairs.push((ww, v));
        }
    }
    pairs.sort_by_key(|v| (v.0, -1 * v.1 as i64));

    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let pairs: Vec<_> = pairs.into_iter().filter(|&v| (!seen.contains(&v.0), seen.insert(v.0)).0 ).collect();
    pairs
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

fn dp1(n: usize, w:usize, vws: Vec<(usize,usize)>) -> usize {
    let r = n/2;
    let pair1 = solve1(w, &vws[0..r]);
    let pair2 = solve1(w, &vws[r..]);
    let ws: Vec<usize> = pair2.iter().map(|v|v.0).collect();
    use ds::BIT;
    let mut bit = BIT::new(pair2.len());

    for (w, v) in pair2 {
        let idx = ws.binary_search(&w).unwrap();
        bit.add(idx+1, v);
    }

    let mut ans = 0;
    for (ww, v) in pair1 {
        let rem = (w-ww)+1;
        let idx = lower_bound(&ws, &rem);
        if idx > 0 {
            let q = bit.query(idx);
            ans = max(ans, v+q);
        } else {
            ans = max(ans, v);
        }
    }
    ans
}

fn dp2(n: usize, w:usize, vws: Vec<(usize,usize)>) -> usize {
    let mut dp = vec![vec![0; w+1]; n+1];
    for i in 0..n {
        for ww in 0..w+1 {
            let (iv, iw) = vws[i];
            dp[i+1][ww] = max(dp[i+1][ww], dp[i][ww]);
            if ww + iw <= w {
                dp[i+1][ww+iw] = max(dp[i+1][ww+iw], dp[i][ww] + iv);
            }
        }
    }
    *dp[n].iter().max().unwrap()
}

fn dp3(n: usize, w:usize, vws: Vec<(usize,usize)>) -> usize {
    let max_v = 1000*200;
    let mut dp = vec![vec![w+2; max_v+1]; n+1];
    dp[0][0] = 0;
    for i in 0..n {
        for cv in 0..max_v+1 {
            let (iv, iw) = vws[i];
            dp[i+1][cv] = min(dp[i][cv], dp[i+1][cv]);
            if cv >= iv {
                dp[i+1][cv] = min(dp[i][cv-iv]+iw, dp[i+1][cv]);
            }
        }
    }

    let mut ans = 0;
    for v in 0..max_v+1 {
        if dp[n][v] <= w {
            ans = max(ans, v);
        }
    }
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
      w: usize,
      vws: [(usize, usize); n],
    }
    let v1 = dp1(n, w, vws.clone());
    let v2 = dp2(n, w, vws.clone());
    if v1 != v2 {
        panic!();
    } else {
        puts!("{}\n", v1);
    }
}
