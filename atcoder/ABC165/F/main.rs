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

pub fn lower_bound<T: Ord>(a: &Vec<T>, x: &T) -> usize {
    use std::cmp::Ordering;
    let mut l = 0;
    let mut r = a.len();
    while l != r {
        let m = l + (r - l) / 2;
        match a[m].cmp(x) {
            Ordering::Less => l = m + 1,
            Ordering::Equal | Ordering::Greater => r = m,
        }
    }
    l
}

const INF: usize = 1 << 32;

fn dfs(cur: usize, parent: usize, edges: &Vec<Vec<usize>>, aa: &Vec<usize>, ans: &mut Vec<usize>, dp: &mut Vec<usize>) {
    ans[cur] = lower_bound(&dp, &INF); // Get the current lis

    for &child in edges[cur].iter() {
        if child != parent {
            let a = aa[child];
            let idx = lower_bound(dp, &a);
            let prev = dp[idx];
            if dp[idx] >= a {
                dp[idx] = a;
            }
            dfs(child, cur, edges, aa, ans, dp);
            dp[idx] = prev;
        }
    }
}

fn main_() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input!{
      n: usize,
      aa: [usize; n],
      vs: [(usize1,usize1); n-1]
    }
    let mut edges = vec![vec![]; n];
    for &(a,b) in vs.iter() {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut dp = vec![INF; n];
    let mut ans = vec![0; n];
    dp[0] = aa[0];
    dfs(0, n, &edges, &aa, &mut ans, &mut dp);
    for a in ans {
        puts!("{}\n", a);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| main_()).unwrap().join().unwrap();
}