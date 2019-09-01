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

const INF:usize= 2 << 30;

fn rec(cur: usize, u: usize, k: usize, cnt: &Vec<usize>, ans: &mut Vec<usize>) -> Vec<usize> {
    // debug!(cur, u, k);
    let mut v = vec![0; 20];
    if cur > u {
        return v;
    }
    let v1 = rec(cur*2, u, k, cnt, ans);
    let v2 = rec(cur*2+1, u, k, cnt, ans);
    v[0] = cnt[cur];

    let mut temp = min(k, cnt[cur]);
    let mut cost = 0;
    for i in 0..19 {
        v[i+1] = v1[i] + v2[i];
        if k - temp > 0 {
            cost += (i+1) * min(k-temp, v[i+1]);
            temp += min(k-temp, v[i+1]);
        }
    }
    if temp == k {
        ans[cur] = cost;
    }
    v
}


fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      k: usize,
      aa: [usize; n],
    }
    let mut m = *aa.iter().max().unwrap();
    let mut cnt = vec![0; m+1];
    let mut ans = vec![INF; m+1];
    for a in aa {
        cnt[a] += 1;
    }
    rec(1, m, k, &cnt, &mut ans);
    let ans = ans.iter().min().unwrap();
    puts!("{}\n", ans);
}
