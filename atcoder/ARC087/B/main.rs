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
      s: chars,
      x: i64,
      y: i64,
    }
    let n = s.len();
    let mut xmoves = vec![];
    let mut ymoves = vec![];

    let mut d = 0;
    let mut move_x = true;
    let mut initial_move = true;
    let mut initial_move_x = false;
    for i in 0..n {
        if s[i] == 'T' {
            if d > 0 {
                if move_x {
                    if initial_move {
                        initial_move_x = true;
                    }
                    xmoves.push(d);
                } else {
                    ymoves.push(d);
                }
            }
            d = 0;
            move_x = !move_x;
            initial_move = false;
        } else {
            d += 1;
        }
    }
    if d > 0 {
        if move_x {
            if initial_move {
                initial_move_x = true;
            }
            xmoves.push(d);
        } else {
            ymoves.push(d);
        }
    }

    let mut dp = vec![vec![false; 2*n+1]; xmoves.len()+1];
    dp[0][n] = true;
    for i in 0..xmoves.len() {
        let d = xmoves[i];
        for j in 0..2*n+1 {
            if j >= d {
                dp[i+1][j] = dp[i+1][j] || dp[i][j-d];
            }
            if !(i == 0 && initial_move_x) {
                if j+d <= 2*n {
                    dp[i+1][j] = dp[i+1][j] || dp[i][j+d];
                }
            }
        }
    }
    if !dp[xmoves.len()][(n as i64+x) as usize] {
        return println!("{}", "No");
    }

    let mut dp = vec![vec![false; 2*n+1]; ymoves.len()+1];
    dp[0][n] = true;
    for i in 0..ymoves.len() {
        let d = ymoves[i];
        for j in 0..2*n+1 {
            if j >= d {
                dp[i+1][j] = dp[i+1][j] || dp[i][j-d];
            }
            if j+d <= 2*n {
                dp[i+1][j] = dp[i+1][j] || dp[i][j+d];
            }
        }
    }

    if !dp[ymoves.len()][(n as i64+y) as usize] {
        println!("{}", "No");
    } else {
        println!("{}", "Yes");
    }
}
