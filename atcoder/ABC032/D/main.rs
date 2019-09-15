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

fn dp1(n: usize, w:usize, vws: Vec<(usize,usize)>) -> usize {
    let mut ans = 0;
    for bit in 0..1<<n {
        let mut v = 0;
        let mut ww = 0;
        for i in 0..n {
            if bit & (1 << i) > 0 {
                v += vws[i].0;
                ww += vws[i].1;
            }
        }
        if ww <= w {
            ans = max(ans, v);
        }
    }
    ans
}

fn dp2(n: usize, w:usize, vws: Vec<(usize,usize)>) -> usize {
    let mut dp = vec![vec![0; w+1]; n+1];
    for i in 0..n {
        for ww in 0..w+1 {
            let (iv, iw) = vws[i];
            dp[i+1][ww] = max(dp[i+1][ww], dp[i][ww]);
            if ww + iw <= w {
                dp[i+1][ww+iw] = max(dp[i+1][ww+iw], dp[i][ww] + iv);
            }
        }
    }
    *dp[n].iter().max().unwrap()
}

fn dp3(n: usize, w:usize, vws: Vec<(usize,usize)>) -> usize {
    let mut dp = vec![vec![0; w+1]; n+1];
    for i in 0..n {
        for ww in 0..w+1 {
            let (iv, iw) = vws[i];
            dp[i+1][ww] = max(dp[i+1][ww], dp[i][ww]);
            if ww + iw <= w {
                dp[i+1][ww+iw] = max(dp[i+1][ww+iw], dp[i][ww] + iv);
            }
        }
    }
    *dp[n].iter().max().unwrap()
}


fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      w: usize,
      vws: [(usize, usize); n],
    }
    let max_v = vws.iter().map(|v|v.0).max().unwrap();
    let max_w = vws.iter().map(|v|v.1).max().unwrap();

    let ans = if n<=30 {
        dp1(n, w, vws)
    } else if max_w <= 1000 {
        dp2(n, w, vws)
    } else {
        dp3(n, w, vws)
    };
    puts!("{}\n", ans);
}
