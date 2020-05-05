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

struct MinStock {
    odd: usize,
    all: usize
}

impl MinStock {
    fn use_odd(&mut self, card: usize) {
        self.odd -= card;
        self.all = min(self.odd, self.all);
    }

    fn use_all(&mut self, card: usize) {
        self.odd -= card;
        self.all -= card;
    }

    fn use_one(&mut self, number: usize, rem: usize) {
        if number%2 == 0 {
            self.odd = min(self.odd, rem);
        }
        self.all = min(self.all, rem);
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let n:usize = sc.read();
    let mut cs = sc.read_vec::<usize>(n);
    let q:usize = sc.read();

    let mut min_stock = MinStock{odd: cs[0], all: cs[0]};

    for i in 0..n {
        if i%2==0 {
            min_stock.odd = min(min_stock.odd, cs[i]);
        }
        min_stock.all = min(min_stock.all, cs[i]);
    }

    let mut card_used = vec![0; n];
    let mut odd_used = 0;
    let mut all_used = 0;

    let mut ans = 0;
    for i in 0..q {
        let s:usize = sc.read();
        match s {
            1 => {
                // 単品販売
                let x:usize = sc.read::<usize>()-1;
                let a:usize = sc.read();

                let total_used = card_used[x] + all_used + if x%2==0 { odd_used } else { 0 };
                // debug!(s, x, total_used);
                if cs[x] >= total_used+a  {
                    card_used[x] += a;
                    ans += a;

                    let rem = cs[x] - total_used - a;
                    min_stock.use_one(x, rem);
                }
            },
            2 => {
                // 奇数をa枚
                let a:usize = sc.read();
                if min_stock.odd >= a {
                    odd_used += a;

                    min_stock.use_odd(a);
                    ans += a * ((n+1)/2);
                }
            }
            3 => {
                // 全部をa枚
                let a:usize = sc.read();
                if min_stock.all >= a {
                    min_stock.use_all(a);

                    all_used += a;
                    let add = a*n;
                    ans += add;
                }
            }
            _ => unreachable!()
        }
    }
    puts!("{}\n", ans);
}

pub struct Scanner<R> {
    pub reader: R,
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
