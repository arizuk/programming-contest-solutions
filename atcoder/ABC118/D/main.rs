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
      mut n: usize,
      m: usize,
      aa: [usize; m],
    }
    let nums = [
        0, 2 , 5 , 5 , 4 , 5 , 6 , 3 , 7 , 6
    ];
    let mut usable = vec![false; 10];
    for a in aa { usable[a] = true; }
    let mut dp = vec![None; n+1];
    dp[0] = Some(0);
    for i in 0..n+1 {
        for j in 1..10 {
            if !usable[j] { continue; }
            if i >= nums[j] {
                dp[i] = max(dp[i], dp[i-nums[j]].map(|v| v + 1));
            }
        }
    }

    let mut ans = vec![];
    while n > 0 {
        for j in (1..10).rev() {
            if !usable[j] { continue; }
            if n >= nums[j] && dp[n-nums[j]] == dp[n].map(|v| v-1) {
                n -= nums[j];
                ans.push(j);
                break;
            }
        }
    }
    println!("{}", ans.iter().map(|v| v.to_string()).collect::<String>());
}
