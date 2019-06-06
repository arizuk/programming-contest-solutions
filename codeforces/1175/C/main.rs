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

fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let t:usize = sc.read();
    for _ in 0..t {
        let n:usize = sc.read();
        let k:usize = sc.read();
        let mut aa: Vec<i64> = sc.read_vec(n);

        let mut l = aa[0];
        let mut r = aa[n-1];
        // debug!(aa);
        loop {
            let mut mid = (l+r)/2;
            let a = get_k(&aa, mid-1, k);
            let b = get_k(&aa, mid, k);
            let c = get_k(&aa, mid+1, k);

            debug!(l, r, mid, k, a, b, c);
            if a < b {
                r = mid;
            } else if c < b {
                l = mid;
            } else {
                debug!(mid, b);
                println!("{}", mid);
                break;
            }
        }
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