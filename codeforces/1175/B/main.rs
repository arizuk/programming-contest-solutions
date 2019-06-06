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
    let l:u64 = sc.read();

    use std::collections::VecDeque;
    let mut stack = VecDeque::new();

    let mut x = 0u64;

    const MAX: u64 = 1 << 32;

    for _ in 0..l {
        let command: String = sc.read();
        if command == "add" {
            if let Some(v) = stack.back() {
                x += v;
                x = min(x, MAX);
            } else {
                x += 1;
            }
        } else if command == "for" {

            let n: u64 = sc.read();
            let v = match stack.back() {
                Some(v) => min(v*n, MAX),
                None => n
            };
            stack.push_back(v);

        } else if command == "end" {
            stack.pop_back();
        }
    }
    if x >= MAX {
        println!("OVERFLOW!!!");
    } else {
        println!("{}", x);
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