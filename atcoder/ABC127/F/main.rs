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
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rev(i64);

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

fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let q: usize = sc.read();

    use std::collections::BinaryHeap;
    let mut lq = BinaryHeap::new();
    let mut rq = BinaryHeap::new();

    let mut lsum = 0i64;
    let mut rsum = 0;
    let mut bsum = 0;

    for _ in 0..q {
        let t:usize = sc.read();
        if t == 1 {
            let a:i64 = sc.read();
            let b:i64 = sc.read();
            bsum += b;

            if lq.len() == 0 {
                lq.push(a);
                lsum += a;
                continue;
            }

            if lq.len() > rq.len() {
                rsum += a;
                rq.push(Rev(a));
            } else {
                lsum += a;
                lq.push(a);
            }

            let mut l = lq.pop().unwrap();
            let Rev(mut r) = rq.pop().unwrap();

            lsum -= l;
            rsum -= r;
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }

            lq.push(l);
            lsum += l;
            rq.push(Rev(r));
            rsum += r;
        } else {
            let x = *lq.peek().unwrap();
            println!(
                "{} {}",
                x,
                bsum + (rsum - rq.len() as i64 *x) + (lq.len() as i64 *x - lsum)
            );
        }
    }
}
