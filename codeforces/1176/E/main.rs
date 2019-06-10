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

fn solve() {
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };
    let t: usize = sc.read();

    for _ in 0..t {
        let n:usize = sc.read();
        let m:usize = sc.read();
        let mut edges = vec![vec![]; n];
        for _ in 0..m {
            let a: usize = sc.read::<usize>() - 1;
            let b: usize = sc.read::<usize>() - 1;
            edges[a].push(b);
            edges[b].push(a);
        }
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        let mut color = vec![0; n];
        q.push_back((0, 1));

        while let Some((node, c)) = q.pop_front() {
            if vis[node] {
                continue;
            }
            vis[node] = true;
            color[node] = c;
            for &next_node in edges[node].iter() {
                if !vis[next_node] {
                    q.push_back((next_node,1-c));
                }
            }
        }
        // debug!(color);
        let mut ans = vec![];
        for i in 0..n {
            if color[i] == 1 {
                ans.push(i+1);
            }
        }
        println!("{}", ans.len());
        for a in ans {
            print!("{} ", a);
        }
        println!();
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