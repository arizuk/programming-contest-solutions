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

pub fn uppper_bound<T: Ord>(a: &Vec<T>, x: &T) -> usize {
    use std::cmp::Ordering;
    let mut l = 0;
    let mut r = a.len();
    while l != r {
        let m = l + (r - l) / 2;
        match a[m].cmp(x) {
            Ordering::Less | Ordering::Equal => l = m + 1,
            Ordering::Greater => r = m,
        }
    }
    l
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      xs: [usize; n],
      l: usize,
      q: usize,
      abs: [(usize1, usize1); q]
    }

    // Doubling
    const INF: usize = 1 << 30;
    let mut next_x = vec![vec![INF; n]; 20];
    for i in 0..n-1 {
        let idx = uppper_bound(&xs, &(xs[i]+l));
        next_x[0][i] = idx-1;
    }

    for p in 1..20 {
        for i in 0..n-1 {
            if next_x[p-1][i] != INF {
                next_x[p][i] = next_x[p-1][ next_x[p-1][i] ];
            }
        }
    }

    for (mut a, mut b) in abs {
        if a>b {
            std::mem::swap(&mut a, &mut b);
        }
        let mut pos = a;
        let mut ans = 0;
        let mut ok = false;
        while a < b {
            let mut ok = n as i64;
            let mut ng = -1;
            while (ok-ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if next_x[mid as usize][a] >= b {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
        }
            for p in 0..20 {
                let nx = next_x[p][pos];
                if nx == b || (p == 0 && b <= nx) {
                    ok = true;
                    ans += 1 << p;
                    break;
                }
                if next_x[p][pos] > b {
                    pos = next_x[p-1][pos];
                    ans += 1 << (p-1);
                    break;
                }
            }
            if ok {
                break;
            }
        }
        puts!("{}\n", ans);
    }
}
