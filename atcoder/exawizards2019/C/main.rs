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
type I = usize;

// [l, r]
fn binary_search<F>(mut l: usize, mut r: usize, f: &F) -> Option<usize>
  where F: Fn(usize) -> bool
  {
    assert!(l <= r);
    r += 1;
    let r_bound = r;
    while r > l {
        let m = l + (r - l) / 2;  // avoid overflow
        if f(m) {
          r = m;
        } else {
          if l == m { break; }
          l = m;
        }
    }
    if r == r_bound { None } else { Some(r) }
}

fn main() {
    input!{
      n: usize,
      q: usize,
      s: chars,
      tds: [(chars, chars); q],
    }

    // 左に落ちるならfalse
    let f = |mut idx| {
        for i in 0..q {
            let t = tds[i].0[0];
            let d = tds[i].1[0];

            if s[idx] != t {
                continue;
            }
            if d == 'L' {
                if idx == 0 {
                    return false;
                }
                idx -= 1;
            } else {
                idx += 1;
                if idx >= n {
                    return true;
                }
            }
        }
        true
    };
    let left_index = binary_search(0, n-1, &f);

    // 右に落ちるならtrue
    let f = |mut idx| {
        for i in 0..q {
            let t = tds[i].0[0];
            let d = tds[i].1[0];

            if s[idx] != t {
                continue;
            }
            if d == 'L' {
                if idx == 0 {
                    return false;
                }
                idx -= 1;
            } else {
                idx += 1;
                if idx >= n {
                    return true;
                }
            }
        }
        false
    };
    let right_index = binary_search(0, n-1, &f);
    debug!(left_index);
    debug!(right_index);

    let mut ans = 0;
    if let Some(l) = left_index {
        ans += l;
    }
    if left_index.is_none() {
        ans = n;
    }
    if let Some(r) = right_index {
        ans += n-r;
    }
    println!("{}", n-ans);
}
