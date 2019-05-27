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

fn rec(flags: &mut Vec<bool>, cur: usize, ps: &Vec<usize>, ss: &Vec<Vec<usize>>, ans: &mut usize) {
    if cur == flags.len() {
        let mut ok = true;
        for i in 0..ps.len() {
            let mut on = 0;
            for &s in ss[i].iter() {
                if flags[s-1] {
                    on += 1;
                }
            }
            if on%2 != ps[i] {
                ok = false;
                break;
            }
        }
        if ok {
            *ans += 1;
        }
        return
    }

    flags[cur] = true;
    rec(flags, cur+1, ps, ss, ans);
    flags[cur] = false;
    rec(flags, cur+1, ps, ss, ans);
}


fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut ss = vec![];
    for _ in 0..m {
        let k: usize = sc.read();
        let s: Vec<usize> = sc.read_vec(k);
        ss.push(s);
    }
    let ps: Vec<usize> = sc.read_vec(m);
    let mut flags = vec![false; n];
    let mut ans = 0;
    rec(&mut flags, 0, &ps, &ss, &mut ans);
    println!("{}", ans);
}
