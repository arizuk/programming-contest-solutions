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
type I = usize;
type Chars = Vec<char>;
const INF: usize = 1 << 20;


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
        buf.parse::<T>()
            .ok()
            .expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

fn update(dp: &mut Vec<Vec<Vec<usize>>>, i:usize, j:usize, k:usize, x:&Chars, y:&Chars, z:&Chars, next: &Vec<Vec<usize>>) {
    dp[i][j][k] = INF;
    if i > 0 && i <= x.len() && dp[i-1][j][k] != INF {
        let c = (x[i-1] as u8 - 'a'  as u8) as usize;
        let next = next[c][ dp[i-1][j][k] ];
        if next != INF {
            dp[i][j][k] = min(dp[i][j][k], next + 1);
        }
    }
    if j > 0 && j <= y.len() && dp[i][j-1][k] != INF {
        let c = (y[j-1] as u8 - 'a'  as u8) as usize;
        let next = next[c][ dp[i][j-1][k] ];
        if next != INF {
            dp[i][j][k] = min(dp[i][j][k], next + 1);
        }
    }
    if k > 0 && k <= z.len() && dp[i][j][k-1] != INF {
        let c = (z[k-1] as u8 - 'a'  as u8) as usize;
        let next = next[c][ dp[i][j][k-1] ];
        if next != INF {
            dp[i][j][k] = min(dp[i][j][k], next + 1);
        }
    }
}

fn solve() {
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let n:usize = sc.read();
    let q:usize = sc.read();
    let s:Vec<char> = sc.chars();

    let mut qs = vec![];
    for _ in 0..q {
        let op: char = sc.read();
        let r: usize = sc.read();
        let mut w = '-';
        if op == '+' {
            w = sc.read();
        }
        qs.push((op, r, w));
    }

    let mut next = vec![vec![INF; n+1]; 26];
    for c in 0..26 {
        for i in (0..n).rev() {
            next[c][i] = next[c][i+1];
            if s[i] == (c as u8 + 'a' as u8) as char {
                next[c][i] = i;
            }
        }
    }

    const M: usize = 251;
    let mut dp = vec![vec![vec![INF; M]; M]; M];
    dp[0][0][0] = 0;

    let mut x: Vec<char> = vec![];
    let mut y: Vec<char> = vec![];
    let mut z: Vec<char> = vec![];

    let mut cnt = 0;
    for &(op, r, w) in qs.iter() {
        if op == '+' {
            if r == 1 {
                x.push(w);
                let i = x.len();
                for j in 0..M {
                    for k in 0..M {
                        update(&mut dp, i, j, k, &x, &y, &z, &next);
                    }
                }
            } else if r == 2 {
                y.push(w);
                let j = y.len();
                for i in 0..M {
                    for k in 0..M {
                        update(&mut dp, i, j, k, &x, &y, &z, &next);
                    }
                }
            } else {
                z.push(w);
                let k = z.len();
                for i in 0..M {
                    for j in 0..M {
                        update(&mut dp, i, j, k, &x, &y, &z, &next);
                    }
                }
            }
        } else {
            if r == 1 {
                x.pop();
            } else if r == 2 {
                y.pop();
            } else {
                z.pop();
            }
        }
        let ans = dp[x.len()][y.len()][z.len()];
        cnt += 1;
        if ans != INF {
            println!("{}", "YES");
        } else {
            println!("{}", "NO");
        }
    }
}

fn main() {
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
