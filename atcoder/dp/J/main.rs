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

fn main() {
    input!{
      n: usize,
      aa: [usize; n],
    }

    let mut xx = 0;
    let mut yy = 0;
    let mut zz = 0;
    for i in 0..n {
        if aa[i] == 1 { xx += 1; }
        if aa[i] == 2 { yy += 1; }
        if aa[i] == 3 { zz += 1; }
    }
    let mut dp = vec![vec![vec![-1.0; n+1]; n+1]; n+1];
    dp[xx][yy][zz] = 0.0;

    for x in (0..n+1).rev() {
        for y in (0..n+1).rev() {
            for z in (0..zz+1).rev() {
                if x + y + z > n {
                    break;
                }
                debug!(x, y, z);
                if dp[x][y][z] != -1.0 {
                    debug!(x, y, z, dp[x][y][z]);
                    if z > 0 {
                        dp[x][y+1][z-1] = dp[x][y][z] + n as f64/(z as f64);
                        debug!(x, y+1, z-1, dp[x][y+1][z-1]);
                    }
                    if y > 0 {
                        dp[x+1][y-1][z] = dp[x][y][z] + n as f64/(y as f64);
                    }
                    if x > 0 {
                        dp[x-1][y][z] = dp[x][y][z] + n as f64/(x as f64);
                    }
                }
            }
        }
    }
    println!("{}", dp[0][0][0]);
}
