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
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    // interactive question

    let mut ans: usize = 0;
    for &bit_shift in [0usize, 7].iter() {
        print!("? ");
        for i in 1..101 {
            print!("{} ", i << bit_shift);
        }
        print!("\n");
        stdout().flush();

        let res: usize = sc.read();
        let mask: usize = ((1 << 7) - 1) << if bit_shift == 0 { 7 } else { 0 };
        ans += res & mask;
        // println!("{:014b}", mask);
        // println!("{:014b}", ( (1 << 7) - 1));
        // println!("{:014b}", ( (1 << 7) - 1) << 7);
    }
    println!("! {}", ans);
    stdout().flush();
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
