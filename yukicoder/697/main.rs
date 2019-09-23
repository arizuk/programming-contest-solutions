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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let h:usize = sc.read();
    let w:usize = sc.read();

    let mut f = vec![];
    for _ in 0..h {
        let r:Vec<u8> = sc.read_vec(w);
        f.push(r);
    }

    use std::collections::VecDeque;
    let mut vis = vec![vec![false; w]; h];
    let mut ans = 0;
    for x in 0..h {
        for y in 0..w {
            if f[x][y] == 0 || vis[x][y] {
                continue;
            }
            ans += 1;
            let mut q = VecDeque::new();
            q.push_back((x,y));

            while q.len() > 0 {
                let (x, y) = q.pop_front().unwrap();
                let x = x as i64;
                let y = y as i64;

                let ds = vec![
                    (x+1, y),
                    (x, y+1),
                    (x-1, y),
                    (x, y-1),
                ];
                for (nx, ny) in ds {
                    if nx < 0 || ny < 0 {
                        continue;
                    }
                    let x = x as usize;
                    let y = y as usize;
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if nx >= h || ny >= w {
                        continue;
                    }

                    if vis[nx][ny] {
                        continue;
                    }
                    if f[nx][ny] == 0 {
                        continue;
                    }
                    vis[nx][ny] = true;
                    q.push_back((nx, ny));
                }
            }
        }
    }
    puts!("{}\n", ans);
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
