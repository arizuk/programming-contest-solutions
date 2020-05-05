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

    let n: usize = sc.read();
    let q: usize = sc.read();

    let mut follow = vec![vec![false; n]; n];
    fn print_table(table: &Vec<Vec<bool>>) {
        let n = table.len();
        for i in 0..n {
            for j in 0..n {
                if table[i][j] {
                    print!("{}", "Y")
                } else {
                    print!("{}", "N");
                }
            }
            println!();
        }
    }


    for _ in 0..q {
        let s:usize = sc.read();
        match s {
            1 => {
                let a:usize = sc.read::<usize>()-1;
                let b:usize = sc.read::<usize>()-1;
                follow[a][b] = true;
            },
            2 => {
                let a:usize = sc.read::<usize>()-1;
                for i in 0..n {
                    if follow[i][a] == true {
                        follow[a][i] = true;
                    }
                }
            },
            3 => {
                let a:usize = sc.read::<usize>()-1;

                let mut new_follows = vec![];
                for i in 0..n {
                    if follow[a][i] == false {
                        continue;
                    }
                    for j in 0..n {
                        if j == a {
                            continue;
                        }
                        if follow[i][j] {
                            new_follows.push((a,j));
                        }
                    }
                }
                for (i, j) in new_follows {
                    follow[i][j] = true;
                }
            },
            _ => panic!(),
        }
        // print_table(&follow);
        // println!("")
    }
    print_table(&follow);
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