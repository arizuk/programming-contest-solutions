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

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      d: usize,
      s: chars,
    }
    let s: Vec<_> = s.iter().map(|&c| (c as u8 - '0' as u8) as usize).collect();
    let n = s.len();
    let mut dp0 = vec![vec![0; d]; 2];
    let mut dp1 = vec![vec![0; d]; 2];

    dp0[0][0] = 1;
    for i in 0..n {
        let idx = i%2;
        let digit = s[i];

        for m in 0..d {
            dp0[1-idx][m] = 0;
            dp1[1-idx][m] = 0;
        }

        for m in 0..d {

            dp0[1-idx][ (m + digit) %d  ] += dp0[idx][m];
            dp0[1-idx][ (m + digit) %d  ] %= MOD;

            for i in 0..10 {
                if i < digit {
                    dp1[1-idx][ (m + i) %d  ] += dp0[idx][m];
                    dp1[1-idx][ (m + i) %d  ] %= MOD;
                }
                dp1[1-idx][ (m + i) %d  ] += dp1[idx][m];
                dp1[1-idx][ (m + i) %d  ] %= MOD;
            }
        }
    }
    let ans = ((dp0[n%2][0] + dp1[n%2][0]) % MOD + MOD - 1) % MOD;
    puts!("{}\n", ans);
}
