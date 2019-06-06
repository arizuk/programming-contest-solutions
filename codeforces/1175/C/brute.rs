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

fn get_k(aa: &Vec<i64>, x: i64, k: usize) -> i64 {
    let mut ds: Vec<i64> = aa.iter().map(|&a| (a - x).abs()).collect();
    ds.sort();
    ds[k]
}

fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let t:usize = sc.read();
    for _ in 0..t {
        let n:usize = sc.read();
        let k:usize = sc.read();
        let mut aa: Vec<i64> = sc.read_vec(n);

        let mut ans = 0;
        let mut min_x = 1 << 60;
        for x in aa[0]..aa[n-1]+1 {
            let mut ds: Vec<i64> = aa.iter().map(|&a| (a - x).abs()).collect();
            ds.sort();
            debug!(x, ds[k]);
            if ds[k] < min_x {
                ans = x;
                min_x = ds[k];
            }
        }
        debug!(ans, min_x);
        println!("{}", ans);
        return;
    }
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