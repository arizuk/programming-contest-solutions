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
      q: usize,
      mut xs: [i64; n],
      cds: [(i64,i64); q],
    }
    xs.sort();
    // debug!(xs);

    let mut acm = vec![0; n+1];
    for i in 0..n {
        acm[i+1] = acm[i] + xs[i];
    }

    for (c, d) in cds {
        let l = max(c-d, 0);
        let r = c+d;
        let ci = lower_bound(&xs, &c);
        let li = lower_bound(&xs, &l);
        let ri = uppper_bound(&xs, &r);

        let mut ans = (n - (ri - li)) as i64 * d;

        // [li, ci)
        ans += (ci-li) as i64 * c - (acm[ci] - acm[li]) as i64;

        // [li, ri)
        ans += (acm[ri] - acm[ci]) - (ri-ci) as i64 * c;
        puts!("{}\n", ans);
    }
}
