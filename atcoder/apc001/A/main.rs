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

#[derive(Debug)]
pub struct UnionFind {
    par: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind { par: vec }
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
    }
}


#[allow(unused_imports)]
use std::cmp::{min, max};
#[allow(unused_imports)]
use std::io::{stdout, stdin, BufWriter, Write};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rev(usize);

impl Ord for Rev {
    fn cmp(&self, other: &Rev) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Rev {
    fn partial_cmp(&self, other: &Rev) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

use std::collections::BinaryHeap;

type Item = (Rev, usize);

fn merge(hs: &mut Vec<BinaryHeap<Item>>, h1:usize,h2:usize) -> usize {
    if hs[h1].len() < hs[h2].len() {
        return merge(hs, h2, h1);
    }

    while let Some((v1,_)) = hs[h2].pop() {
        hs[h1].push((v1, h1));
    }
    return h1
}


fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      m: usize,
      aa: [usize; n],
      xys: [(usize,usize); m]
    }

    let mut uf = UnionFind::new(n);
    for (x,y) in xys {
        uf.unite(x,y);
    }

    let mut groups = vec![];
    for i in 0..n {
        groups.push(uf.find(i));
    }
    groups.sort();
    groups.dedup();

    let k = groups.len();
    if k == 1 {
        return puts!("{}\n", 0);
    }
    if n < 2 * (n-m-1) {
        return puts!("{}\n", "Impossible");
    }

    let mut group_nodes = vec![vec![]; k];
    let mut heap = BinaryHeap::new();

    for i in 0..n {
        let a = aa[i];
        let g = uf.find(i);
        let g = groups.binary_search(&g).unwrap();
        group_nodes[g].push(a);
    }

    let mut ans = 0;
    for i in 0..k {
        group_nodes[i].sort();
        ans += group_nodes[i][0];
        for j in 1..group_nodes[i].len() {
            heap.push(Rev(group_nodes[i][j]));
        }
    }
    let rem = 2 * (n-m-1) - k;
    for _ in 0..rem {
        let Rev(v) = heap.pop().unwrap();
        ans += v;
    }
    puts!("{}\n", ans);
}
