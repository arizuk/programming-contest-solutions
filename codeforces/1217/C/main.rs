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

fn solve(s: Vec<char>) -> usize {
    let n = s.len();
    let mut ans = 0;
    let mut prevs = vec![-1; n];
    let mut prev = -1;
    for i in 0..n {
        if s[i] == '1' {
            prevs[i] = prev;
            prev = i as i64;
        }
    }

    for i in 0..n {
        if s[i] == '0' {
            continue;
        }

        let mut value = 0;
        for j in 0..20 {
            if i+j >= n {
                break;
            }
            value *= 2;
            if s[i+j] == '1' {
                value += 1;
            }
            if i+j+1 < value {
                continue;
            }
            if prevs[i] < (i+j+1-value) as i64 {
                ans += 1;
            }
        }
    }
    ans
}

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
        let s = sc.chars();
        let ans = solve(s);
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
