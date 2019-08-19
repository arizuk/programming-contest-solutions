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
      h: usize,
      w: usize,
      k: usize,
    }

    const MOD: usize = 1e9 as usize + 7;
    let mut dp = vec![vec![0; w]; h+1];
    dp[0][0] = 1;

    if w == 1 {
        return puts!("{}\n", 1);
    }

    for i in 0..h {
        for bit in 0..(1 << (w-1)) {
            let lines: Vec<bool> = (0..w-1).map(|i| (bit & (1 << i) > 0)).collect();
            if lines.windows(2).any(|p| p[0] && p[1]) {
                continue;
            }
            for cur in 0..w {
                if cur < w-1 && lines[cur] {
                    dp[i+1][cur+1] += dp[i][cur];
                    dp[i+1][cur+1] %= MOD;
                } else if cur > 0 && lines[cur-1] {
                    dp[i+1][cur-1] += dp[i][cur];
                    dp[i+1][cur-1] %= MOD;
                } else {
                    dp[i+1][cur] += dp[i][cur];
                    dp[i+1][cur] %= MOD;
                }
            }
        }
    }
    puts!("{}\n", dp[h][k-1]);
}
