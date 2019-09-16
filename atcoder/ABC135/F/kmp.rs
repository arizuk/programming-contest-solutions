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

// KMP法
fn make_kmp_table(s: &Vec<char>) -> Vec<i64> {
    let n = s.len();
    let mut t = vec![0; n+1];
    t[0] = -1;

    let mut pos = 0;
    let mut i = 2;
    while i <= n {
        let prev = s[i-1];
        if prev == s[pos] {
            pos += 1;
            t[i] = pos as i64;
            i += 1;
        } else if pos > 0 {
            // TODO: この処理の意味
            pos = t[pos] as usize;
        } else {
            i += 1;
        }
    }
    t
}

#[derive(Debug)]
pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        let size = vec![1; n];
        UnionFind { par: vec, size: size }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        } else {
            let par = self.par[x];
            let res = self.find(par);
            self.par[x] = res;
            res
        }
    }
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
    pub fn unite(&mut self, a: usize, b: usize) {
        let apar = self.find(a);
        let bpar = self.find(b);
        self.par[apar] = bpar;
        self.size[bpar] = self.size[apar]  + self.size[bpar];
    }
}

fn calc_maximum_length(partial: &Vec<bool>, n: usize, m: usize) -> i64 {
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        let ni = (i + m) % n;
        if partial[i] && partial[ni] {
            if uf.same(i, ni) {
                return -1;
            } else {
                uf.unite(i, ni);
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if partial[i] {
            let par = uf.find(i);
            ans = max(ans, uf.size[par]);
        }
    }
    ans as _
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      s: chars,
      t: chars,
    }

    let n = s.len();
    let m = t.len();
    let tbl = make_kmp_table(&t);
    // debug!(tbl);

    let mut s_idx = 0;
    let mut offset = 0;
    let mut partial = vec![false; n];
    while s_idx < n {
        let mut t_idx = offset;
        let mut ok = true;
        while t_idx < m {
            if s[(s_idx + t_idx) % n] != t[t_idx] {
                ok = false;
                break;
            }
            t_idx += 1;
        }
        if ok {
            partial[s_idx] = true;
        }
        s_idx = ((s_idx + t_idx) as i64 - tbl[t_idx]) as usize;
        offset = max(tbl[t_idx], 0) as usize;
    }

    let ans = calc_maximum_length(&partial, n, m);
    puts!("{}\n", ans);
}
