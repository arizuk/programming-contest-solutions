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

fn solve() {
    input!{
      n: usize,
      cs: [u16; n]
    }
    let mut a = vec![];
    for i in 1..n {
        if cs[i] != cs[i-1] {
            a.push(cs[i-1]);
        }
    }
    a.push(cs[n-1]);

    let n = a.len();
    let mut dp = [[[0u16; 2]; 5001]; 5001];

    let mv = |i, j| -> u16 {
        if a[i] == a[j] { 0 } else { 1 }
    };

    for len in 1..n {
        for l in 0..n-len {
            let r = l + len;
            dp[l][r][0] = n as u16;
            dp[l][r][1] = n as u16;

            dp[l][r][0] = min(dp[l+1][r][0] + mv(l, l+1), dp[l][r][0]);
            dp[l][r][0] = min(dp[l+1][r][1] + mv(l, r), dp[l][r][0]);

            dp[l][r][1] = min(dp[l][r-1][0] + mv(l, r), dp[l][r][1]);
            dp[l][r][1] = min(dp[l][r-1][1] + mv(r-1, r), dp[l][r][1]);
        }
    }
    println!("{}", min(dp[0][n-1][0], dp[0][n-1][1]));
}

fn main() {
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}