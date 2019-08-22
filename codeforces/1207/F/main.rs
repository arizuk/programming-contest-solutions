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

    let mut dp = vec![vec![0; 700]; 700];

    const MAX: usize = 500001;
    let mut table = vec![0; MAX];

    let q:usize= sc.read();
    for _ in 0..q {
        let t:usize = sc.read();
        if t == 1 {
            let x:usize = sc.read();
            let y:i64 = sc.read();

            for modulo in 1..700 {
                let idx = x % modulo;
                dp[modulo][idx] += y;
            }
            table[x] += y;

        } else {
            let x:usize = sc.read();
            let y:usize = sc.read();

            if x < 700 {
                puts!("{}\n", dp[x][y]);
            } else {
                let mut cur = y;
                let mut ans = 0;
                while cur < MAX {
                    ans += table[cur];
                    cur += x;
                }
                puts!("{}\n", ans);
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