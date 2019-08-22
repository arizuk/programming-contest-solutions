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

    let t:usize = sc.read();
    for _ in 0..t {
        let n:usize = sc.read();
        let a:u64 = sc.read();
        let b:u64 = sc.read();

        let s: Vec<_> = sc.chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect();

        let mut ans = (n+1) as u64 * b + a * n as u64;
        let mut cur = 0;

        while !(cur == s.len() || s[cur] == 1) {
            cur += 1;
        }
        if cur == s.len() {
            puts!("{}\n", ans);
            continue;
        }
        ans += a;

        loop {
            let mut ones = 0;
            let mut zeros = 0;

            while !(cur == s.len() || s[cur] == 0) {
                ones += 1;
                cur += 1;
            }

            while !(cur == s.len() || s[cur] == 1) {
                zeros += 1;
                cur += 1;
            }

            // debug!(ones, zeros, cur);
            if cur == s.len() {
                ans += (ones + 1) * b + a;
                break;
            } else {
                let p1 = (ones + zeros) * b;
                let p2 = (ones + 1) * b + 2 * a;
                ans += min(p1, p2);
            }
        }
        puts!("{}\n", ans);
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