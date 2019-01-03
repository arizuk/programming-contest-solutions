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

#[allow(unused_imports)]
use std::io::Write;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    input!{
      n: usize,
      m: usize,
      ps: [usize; n],
      xys: [(usize, usize); m]
    }

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (x, y) in xys {
        {
            let e = map.entry(x).or_insert(vec![]);
            e.push(y);
        }
        {
            let e = map.entry(y).or_insert(vec![]);
            e.push(x);
        }
    }

    let f = |n, t| -> bool {
        let mut seen: HashSet<usize> = HashSet::new();
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(n);
        while let Some(n) = q.pop_front() {
            if n == t {
                return true;
            }

            if seen.contains(&n) {
                continue;
            }
            seen.insert(n);

            if let Some(vs) = map.get(&n) {
                for &v in vs {
                    q.push_back(v);
                }
            }
        }
        false
    };

    let mut ans = 0;
    for i in 0..n {
        let p = ps[i];
        if f(p, i+1) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
