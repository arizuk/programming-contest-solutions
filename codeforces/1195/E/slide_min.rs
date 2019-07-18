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
        ($($format:tt)*) => (writeln!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      a: usize,
      b: usize,
      g0: u64,
      x: u64,
      y: u64,
      z: u64,
    }

    let mut hs = vec![vec![0; m]; n];
    let mut g = g0;
    for i in 0..n*m {
        let a = i/m;
        let b = i%m;
        hs[a][b] = g;
        g = (g * x  + y) % z;
    }

    // debug!(n, m, a, b, hs);

    let mut min_table = vec![];

    // スライド最小値
    use std::collections::VecDeque;
    for i in 0..n {
        let mut q: VecDeque<(usize,usize)> = VecDeque::new();
        let mut row_mins = vec![];
        for j in 0..m {
            let h = hs[i][j];

            while q.len() > 0 {
                let (x, y) = *q.back().unwrap();
                if h <= hs[x][y] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back((i, j));

            if j < b-1 {
                continue;
            }
            if j >= b {
                let (_,y) = *q.front().unwrap();
                if y == j-b {
                    q.pop_front();
                }
            }

            // debug!(i, j, q);
            let (x,y) = *q.front().unwrap();
            row_mins.push(hs[x][y]);
        }
        min_table.push(row_mins);
    }

    let mut ans = 0;
    for j in 0..m-b+1 {
        let mut q: VecDeque<(usize,usize)> = VecDeque::new();
        for i in 0..n {
            let h = min_table[i][j];

            while q.len() > 0 {
                let (x, y) = *q.back().unwrap();
                if h <= min_table[x][y] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back((i, j));

            if i < a-1 {
                continue;
            }
            if i >= a {
                let (x,_) = *q.front().unwrap();
                if x == i-a {
                    q.pop_front();
                }
            }

            let (x,y) = *q.front().unwrap();
            ans += min_table[x][y];

        }
    }
    puts!("{}", ans);
}