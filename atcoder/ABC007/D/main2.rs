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
      mut a: usize,
      mut b: usize,
    }
    const MAX_D: usize = 20;

    let mut aa = vec![0; MAX_D];
    let mut bb = vec![0; MAX_D];

    let ttl = b-a + 1;
    let mut cur = 0;
    while a > 0 {
        aa[MAX_D-cur-1] = a%10;
        a /= 10;
        cur += 1;
    }
    let mut cur = 0;
    while b > 0 {
        bb[MAX_D-cur-1] = b%10;
        b /= 10;
        cur += 1;
    }

    let mut dp = vec![vec![vec![0; 2];2]; MAX_D+1];
    dp[0][0][0] = 1;
    for i in 0..MAX_D {
        let d = i;

        for &flag_a in [0, 1].iter() {
            for &flag_b in [0, 1].iter() {
                if flag_a==1 && flag_b==1 {
                    dp[i+1][1][1] += dp[i][1][1] * 8;


                } else if flag_a== 1 {

                    for digit in 0..bb[d] {
                        if digit != 4 && digit != 9 {
                            dp[i+1][1][1] += dp[i][1][0];
                        }
                    }

                    if bb[d] != 4 && bb[d] != 9 {
                        dp[i+1][1][0] += dp[i][1][0];
                    }
                } else if flag_b==1 {

                    for digit in aa[d]+1..10 {
                        if digit != 4 && digit != 9 {
                            dp[i+1][1][1] += dp[i][0][1];
                        }
                    }
                    if aa[d] != 4 && aa[d] != 9 {
                        dp[i+1][0][1] += dp[i][0][1];
                    }
                } else {
                    if aa[d] > bb[d] {
                        continue;
                    }

                    for digit in aa[d]+1..bb[d] {
                        if digit != 4 && digit != 9 {
                            dp[i+1][1][1] += dp[i][0][0];
                        }
                    }

                    if aa[d] == bb[d] {
                        if aa[d] != 4 && aa[d] != 9 {
                            dp[i+1][0][0] += dp[i][0][0];
                        }
                        continue;
                    }

                    if aa[d] != 4 && aa[d] != 9 {
                        dp[i+1][0][1] += dp[i][0][0];
                    }
                    if bb[d] != 4 && bb[d] != 9 {
                        dp[i+1][1][0] += dp[i][0][0];
                    }
                }
            }
        }

        // for &flag_a in [0, 1].iter() {
        //     for &flag_b in [0, 1].iter() {
        //         let v = dp[i+1][flag_a][flag_b];
        //         // debug!(i, flag_a, flag_b, aa[d], bb[d], v);
        //     }
        // }
    }

    let mut ans = 0;
    for &flag_a in [0, 1].iter() {
        for &flag_b in [0, 1].iter() {
            ans += dp[MAX_D][flag_a][flag_b];
        }
    }
    puts!("{}\n", ttl-ans);
}
