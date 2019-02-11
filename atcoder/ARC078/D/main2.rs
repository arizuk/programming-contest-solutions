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

const B: usize = 1;
const W: usize = 2;

fn dfs(edges: &[Vec<usize>], i: usize, t: usize, r: &mut Vec<usize>) -> bool {
    if i == t {
        return true;
    }

    for &n in edges[i].iter() {
        if r.len() >= 2 && r[r.len()-2] == n {
            continue;
        }

        r.push(n);
        if dfs(edges, n, t, r) {
            return true;
        }
        r.pop();
    }
    false
}

fn dfs2(edges: &[Vec<usize>], colors: &[usize], i: usize, seen: &mut Vec<bool>, c: usize) {
    if seen[i] {
        return;
    }
    seen[i] = true;
    for &n in edges[i].iter() {
        if colors[n] == c {
            continue;
        }
        dfs2(edges, colors, n, seen, c);
    }
}


fn main() {
    input!{
      n: usize,
      abs: [(usize, usize); n-1],
    }

    let mut edges = vec![vec![]; n];
    for &(a, b) in abs.iter() {
        edges[a-1].push(b-1);
        edges[b-1].push(a-1);
    }

    let mut colors = vec![0; n];
    colors[0] = B;
    colors[n-1] = W;

    let mut r = vec![0];
    dfs(&edges, 0, n-1, &mut r);

    {
        let mut i = 0;
        let mut j = r.len()-1;
        loop {
            i += 1;
            if i >= j {
                break;
            }
            colors[r[i]] = B;
            j -= 1;
            if i >= j {
                break;
            }
            colors[r[j]] = W;
        }
    }
    let mut black = vec![false; n];
    dfs2(&edges, &colors, 0, &mut black, W);

    let mut b = 0;
    for i in 0..n {
        if black[i] {
            b += 1;
        }
    }
    if b > (n-b) {
        println!("{}", "Fennec");
    } else {
        println!("{}", "Snuke");
    }
}
