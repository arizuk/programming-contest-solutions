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
      n: usize,
      e: [chars; n],
      m: usize,
      ps: [usize1; m],
    }

    const INF: u64 = 1 << 50;
    let mut cs = vec![vec![INF; n]; n];
    for from in 0..n {
        for to in 0..e[from].len() {
            if e[from][to] == '1' {
                cs[from][to] = 1;
            }
        }
    }

    for i in 0..n {
        // debug!(cs[i]);
        cs[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                cs[i][j] = min(cs[i][j], cs[i][k] + cs[k][j]);
            }
        }
    }

    let v1 = ps[0];
    let mut dist = 1;
    let mut ans = vec![];
    ans.push(v1);

    for i in 1..m {
        let cur = ps[i];
        let prev = ans[ans.len()-1];
        if cs[prev][cur] != dist {
            ans.push(ps[i-1]);
            dist = 1;
        }
        dist += 1;
    }
    ans.push(ps[m-1]);

    puts!("{}\n", ans.len());
    for i in 0..ans.len() {
        if i == ans.len() - 1 {
            puts!("{}", ans[i]+1);
        } else {
            puts!("{} ", ans[i]+1);
        }
    }
    puts!("\n")
}
