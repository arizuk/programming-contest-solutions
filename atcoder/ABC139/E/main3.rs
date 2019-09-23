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

fn encode(a: usize, b:usize, n: usize) -> usize {
    if a > b {
        n*b + a
    } else {
        n*a + b
    }
}

#[derive(Debug)]
pub struct Graph {
    n: usize,
    edges: Vec<Vec<usize>>,
}
impl Graph {
    pub fn topological_sort(&self) -> i64 {
        let n = self.n;
        let edges = &self.edges;

        let mut h = vec![0; n];
        for i in 0..n {
            for &t in edges[i].iter() {
                h[t] += 1;
            }
        }

        use std::collections::VecDeque;
        let mut st = VecDeque::new();
        for i in 0..n {
            if h[i] == 0 {
                st.push_back((i,0));
            }
        }

        let mut ans = 0;

        while let Some((i, d)) = st.pop_front() {
            ans = max(ans, d);

            for &t in edges[i].iter() {
                h[t] -= 1;
                if h[t] == 0 {
                    st.push_back((t, d+1));
                }
            }
        }

        for i in 0..n {
            if h[i] != 0 {
                return -1;
            }
        }
        ans+1
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      aa: [[usize1; n-1]; n],
    }

    let mut e = vec![vec![]; n*n];
    for a in 0..n {
        let b0 = aa[a][0];
        let mut from = encode(a, b0, n);
        for j in 1..n-1 {
            let b = aa[a][j];
            let to = encode(a, b, n);
            e[from].push(to);
            from = to;
        }
    }
    for v in e.iter_mut() {
        v.sort();
        v.dedup();
    }

    let g = Graph {
        n: n*n,
        edges: e,
    };

    let ans = g.topological_sort();
    puts!("{}\n", ans);
}
