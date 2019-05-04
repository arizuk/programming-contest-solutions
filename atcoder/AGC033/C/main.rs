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
type I = usize;

use std::collections::VecDeque;

fn dfs(i: usize, prev: usize, edges: &Vec<Vec<usize>>, lg: &mut Vec<usize>) -> usize {
    let mut longest = 0;
    for &next in edges[i].iter() {
        if next != prev {
            longest = max(longest, dfs(next, i, edges, lg));
        }
    }
    lg[i] = longest + 1;
    return lg[i]
}

fn long(i: usize, prev: usize, edges: &Vec<Vec<usize>>, lg: &Vec<usize>, ans: &mut usize) {
    let mut temps = vec![];
    for &next in edges[i].iter() {
        if next != prev {
            temps.push(lg[next]);
        }
    }
    temps.sort();
    temps.reverse();
    let mut temp = 0;

    if temps.len() > 0 {
        temp += temps[0];
    }
    if temps.len() > 1 {
        temp += temps[1];
    }
    *ans = max(*ans, temp+1);

    for &next in edges[i].iter() {
        if next != prev {
            long(next, i, edges, lg, ans);
        }
    }
}


fn main() {
    input!{
      n: usize,
      abs: [(usize, usize); n-1],
    }

    let mut edges = vec![vec![]; n];
    let mut longest = vec![0; n];
    for &(a, b) in abs.iter() {
        edges[a-1].push(b-1);
        edges[b-1].push(a-1);
    }
    dfs(0, n, &edges, &mut longest);
    let mut ans = 0;
    long(0, n, &edges, &longest, &mut ans);

    let mut dp = vec![false; max(5, n+1)];
    dp[1] = true;
    dp[2] = false;
    dp[3] = true;
    dp[4] = true;
    for i in 5..n+1 {
        if dp[i-1] && dp[i-2] {
            dp[i] = false;
        } else {
            dp[i] = true;
        }
    }
    if dp[ans] {
        println!("{}", "First");
    } else {
        println!("{}", "Second");
    }
}
