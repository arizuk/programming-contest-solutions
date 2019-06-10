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
        let aa: Vec<usize> = sc.read_vec(n);
        let mut cnt = vec![0; 3];
        for a in aa {
            cnt[a%3] += 1;
        }
        let mut ans = cnt[0];
        let num = min(cnt[1], cnt[2]);
        cnt[1] -= num;
        cnt[2] -= num;
        ans += num;
        ans += cnt[1]/3;
        ans += cnt[2]/3;
        println!("{}", ans);
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