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

fn check(e: &Vec<Vec<usize>>, cur: usize, par: usize, c: usize, cs: &mut Vec<Option<usize>>) -> bool {
    if let Some(cur_c) = cs[cur] {
        return cur_c == c
    }

    cs[cur] = Some(c);
    for &nd in e[cur].iter() {
        if nd != par {
            if !check(e, nd, cur, 1-c, cs) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      ss: [chars; n],
    }

    const INF: usize = 1 << 50;
    let mut dist = vec![vec![INF; n]; n];

    let mut e = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if ss[i][j] == '1' {
                e[i].push(j);
                e[j].push(i);

                dist[i][j] = 1;
                dist[j][i] = 1;
            }
        }
    }

    let mut cs = vec![None; n];
    let ok = check(&e, 0, n, 0, &mut cs);
    if !ok {
        return puts!("{}\n", -1);
    }

    for i in 0..n {
        dist[i][i] = 0;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = min(dist[i][k] + dist[k][j], dist[i][j]);
            }
        }
    }


    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans = max(ans, dist[i][j]);
        }
    }
    puts!("{}\n", ans+1);
}
