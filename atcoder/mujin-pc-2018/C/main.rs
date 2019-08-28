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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      ss: [chars; n],
    }

    let mut left = vec![vec![0; m]; n];
    let mut right = vec![vec![0; m]; n];
    let mut top = vec![vec![0; m]; n];
    let mut bottom = vec![vec![0; m]; n];

    for i in 0..n {
        let mut cur = 0;
        for j in 0..m {
            if ss[i][j] == '#' {
                cur = 0;
            } else {
                left[i][j] = cur;
                cur += 1;
            }
        }
    }

    for i in 0..n {
        let mut cur = 0;
        for j in (0..m).rev() {
            if ss[i][j] == '#' {
                cur = 0;
            } else {
                right[i][j] = cur;
                cur += 1;
            }
        }
    }

    for j in 0..m {
        let mut cur = 0;
        for i in 0..n {
            if ss[i][j] == '#' {
                cur = 0;
            } else {
                top[i][j] = cur;
                cur += 1;
            }
        }
    }

    for j in 0..m {
        let mut cur = 0;
        for i in (0..n).rev() {
            if ss[i][j] == '#' {
                cur = 0;
            } else {
                bottom[i][j] = cur;
                cur += 1;
            }
        }
    }

    // for i in 0..n {
    //     debug!(i, left[i]);
    //     debug!(i, right[i]);
    //     // debug!(top[i]);
    //     // debug!(bottom[i]);
    // }


    let mut ans = 0u64;
    for i in 0..n {
        for j in 0..m {
            ans += left[i][j] * bottom[i][j];
            ans += top[i][j] * left[i][j];
            ans += right[i][j] * top[i][j];
            ans += bottom[i][j] * right[i][j];
        }
    }
    puts!("{}\n", ans);
}
