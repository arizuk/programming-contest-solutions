#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}

macro_rules! puts {
    ($($a:expr),*) => {
        writeln!(out, $($a), *).unwrap();
    }
}


#[allow(unused_imports)]
use std::cmp::{min, max};
use std::io::{stdout, BufWriter, Write};

fn solve() {
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
        let aa: Vec<usize> = sc.read_vec(n);
        let bb: Vec<usize> = sc.read_vec(n);

        let mut cur = 0;
        let mut ok = true;
        while cur < n {
            let si = cur;
            let s = bb[cur];

            while cur < n && s == bb[cur] {
                cur += 1;
            }

            // 全部同じなのどどっちでよ良い
            if cur == n {
                cur -= 1;
            }
            if bb[cur] > s {
                // ソートされている場合
                while cur < n-1 && bb[cur] <= bb[cur+1] {
                    cur += 1;
                }
                use std::collections::HashMap;
                let mut map = HashMap::new();
                // let mut asub: Vec<_> = (si..cur+1).map(|i|aa[i]).collect();
                // asub.sort();
                let bsub = &bb[si..cur+1];
                debug!(si, cur, bsub);
                for i in si..cur+1 {
                    *map.entry(bb[i]).or_insert(0) += 1;
                }

                for i in si..cur+1 {
                    if !map.contains_key(&aa[i]) {
                        ok = false;
                        break;
                    }
                    let e = map.entry(aa[i]).or_insert(0);
                    if *e == 0 {
                        ok = false;
                        break;
                    }
                    *e -= 1;
                }
            } else {
                while cur < n-1 && bb[cur] >= bb[cur+1] {
                    cur += 1;
                }
                let bsub = &bb[si..cur+1];
                debug!(si, cur, bsub);

                for i in si..cur+1 {
                    if bb[i] != aa[i] {
                        ok = false;
                        break;
                    }
                }
            }

            if !ok {
                break;
            }
            cur += 1;
        }

        if ok {
            puts!("YES\n")
        } else {
            puts!("NO\n")
        }
    }
}


fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
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