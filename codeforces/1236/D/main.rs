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

    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let n:usize = sc.read();
    let m:usize = sc.read();
    let k:usize = sc.read();

    use std::collections::HashSet;
    use std::collections::BTreeSet;
    let mut set = HashSet::new();

    let mut rows = vec![BTreeSet::new(); n];
    let mut cols = vec![BTreeSet::new(); m];

    for _ in 0..k {
        let x:usize = sc.read::<usize>() - 1;
        let y:usize = sc.read::<usize>() - 1;
        set.insert((x,y));
        rows[x].insert(y);
        cols[y].insert(x);
    }

    let mut cx = 0;
    let mut cy = 0;
    let mut mode = 1;
    let mut h_l = 0;
    let mut h_u = n;

    let mut w_l = 0;
    let mut w_u = m;

    for _ in 0..5 {
        debug!(cx, cy, mode);
        match mode {
            1 => {
                let mut iter = rows[cx].range(cy+1..w_u);
                if let Some(y) = iter.next() {
                } else {
                    cx = w_u-1;
                }
                w_u = cx;
                mode = 2;
            },
            2 => {
                let mut iter = rows[cy].range(cx+1..h_u);
                if let Some(y) = iter.next() {
                } else {
                    cy = h_u-1;
                }
                h_u = cy;
                mode = 3;
            },
            3 => {
                let mut iter = rows[cx].range(w_l..cy).rev();
                if let Some(x) = iter.next() {
                } else {
                    cx = w_l;
                }
                w_l = cx+1;
                mode = 4;
            },
            4 => {
                let mut iter = rows[cy].range(h_l..cx).rev();
                if let Some(y) = iter.next() {
                } else {
                    cy = w_l;
                }
                w_l = cy+1;
                mode = 1;
            }
            _ => unreachable!()
        }
    }
    puts!("{}\n", "Yes");
}

pub struct Scanner<R> {
    reader: R,
}
impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf: String = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect();
        buf.parse::<T>().ok().expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
