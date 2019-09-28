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

    let n:usize = sc.read();
    let m:usize = sc.read();

    const INF: usize = 1 << 60;

    let max_v = 1 << n;
    let mut dp = vec![INF; max_v];
    dp[0] = 0;
    for _ in 0..m {
        let a: usize = sc.read();
        let b: usize = sc.read();
        let cc: Vec<usize> = sc.read_vec(b);

        let mut cur = 0;
        for &c in cc.iter() {
            cur |= 1 << (c-1);
        }
        // println!("{:?} {:b}", cc, cur);
        for bit in 0..max_v {
            let nb = bit | cur;
            dp[nb] = min(dp[nb], dp[bit] + a);
        }
    }
    if dp[max_v-1] == INF {
        puts!("{}\n", -1);
    } else {
        puts!("{}\n", dp[max_v-1]);
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

