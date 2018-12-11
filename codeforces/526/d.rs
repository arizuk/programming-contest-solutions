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
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

fn dfs(edges: &Vec<Vec<(usize, i64)>>, dp: &mut Vec<i64>, ws: &Vec<i64>, u: usize, pre: usize, ans: &mut i64) {
    dp[u] = ws[u];
    *ans = max(*ans, dp[u]);

    let mut q: Vec<i64> = vec![];
    for &(v, c) in &edges[u] {
        if v == pre { continue; }
        dfs(edges, dp, ws, v, u, ans);
        dp[u] = max(ws[u] + dp[v] - c, dp[u]);
        *ans = max(*ans, dp[u]);
        if dp[v] - c > 0 {
            q.push(dp[v] - c)
        }
    }
    q.sort();
    q.reverse();

    if q.len() >= 2 {
        let tmp = ws[u] + q[0] + q[1];
        *ans = max(*ans, tmp);
    }
}

fn solve() {
    input!{
      n: usize,
      ws: [i64; n],
      uvcs: [[usize; 3]; n-1]
    }
    let mut edges: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    let mut dp = vec![0i64; n];

    for uvc in uvcs {
        let u = uvc[0] - 1;
        let v = uvc[1] - 1;
        let c = uvc[2] as i64;
        edges[u].push((v, c));
        edges[v].push((u, c));
    }

    let mut ans = 0;
    dfs(&edges, &mut dp, &ws, 0, 0, &mut ans);
    println!("{}", ans);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024) // 32 MBのスタックサイズ
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
