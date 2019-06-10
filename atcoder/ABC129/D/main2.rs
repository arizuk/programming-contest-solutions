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

fn main() {
    input!{
      h: usize,
      w: usize,
      ss: [chars; h],
    }
    let mut col = vec![vec![0; w]; h];
    let mut row = vec![vec![0; w]; h];

    for i in 0..h {
        // wについて調べる
        let mut idx: i64 = -1;
        let mut num = 0;
        for j in 0..w {
            if ss[i][j] == '#' {
                // debug!(i, j, idx, num);
                if idx >= 0 {
                    for k in (idx as usize)..j {
                        row[i][k] = num;
                    }
                }
                num = 0;
                idx = -1;
            } else {
                // debug!(i, j, idx, num);
                if idx < 0 {
                    num = 0;
                    idx = j as i64;
                }
                num += 1;
            }
        }

        if idx >= 0 {
            for k in (idx as usize)..w {
                row[i][k] = num;
            }
        }
        // debug!(i, row[i]);
    }

    for j in 0..w {
        // wについて調べる
        let mut idx: i64 = -1;
        let mut num = 0;
        for i in 0..h {
            if ss[i][j] == '#' {
                // debug!(i, j, idx, num);
                if idx >= 0 {
                    for k in (idx as usize)..i {
                        col[k][j] = num;
                    }
                }
                num = 0;
                idx = -1;
            } else {
                // debug!(i, j, idx, num);
                if idx < 0 {
                    num = 0;
                    idx = i as i64;
                }
                num += 1;
            }
        }

        if idx >= 0 {
            for k in (idx as usize)..h {
                col[k][j] = num;
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == '.' {
                ans = max(ans, col[i][j] + row[i][j] - 1);
            }
        }
    }
    println!("{}", ans);
}
