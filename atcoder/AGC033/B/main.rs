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

    ($next:expr, isize1) => {
        read_value!($next, isize) - 1
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

fn main() {
    input!{
      h: isize,
      w: isize,
      n: usize,
      sr: isize1,
      sc: isize1,
      ss: chars,
      ts: chars,
    }

    let mut left = 0;
    let mut right = w-1;
    let mut top = 0;
    let mut buttom = h-1;

    for i in (0..n).rev() {
        let s = ss[i];
        let t = ts[i];

        if i != n-1 {
            match t {
                'U' => buttom = min(buttom+1, h-1),
                'D' => top = max(top-1, 0),
                'R' => left = max(left-1, 0),
                'L' => right = min(right+1, w-1),
                _ => unreachable!()
            }
        }

        match s {
            'U' => top = top+1,
            'D' => buttom = buttom-1,
            'R' => right = right-1,
            'L' => left = left+1,
            _ => unreachable!()
        }

        if top >= h || buttom < 0 || left >= w || right < 0 {
            return println!("{}", "NO");
        }
        if top > buttom || left > right {
            return println!("{}", "NO");
        }
    }

    if sr >= top && sr <= buttom && sc >= left && sc <= right {
        println!("{}", "YES")
    } else {
        println!("{}", "NO")
    }
}
