
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
        let mut x:i64 = sc.read();

        let mut dhs = vec![];
        for _ in 0..n {
            let d:i64 = sc.read();
            let h:i64 = sc.read();
            dhs.push((d,h,d-h));
        }
        let max_d: i64 = dhs.iter().map(|v|v.0).max().unwrap();
        dhs.sort_by_key(|v| v.2);
        let max_diff: i64 = dhs.iter().map(|v|v.2).max().unwrap();

        if max_d >= x {
            puts!("{}\n", 1);
            continue;
        }

        if max_diff <= 0 {
            puts!("{}\n", -1);
            continue;
        }

        let ans = ( x - max_d + max_diff - 1 ) / max_diff + 1;
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