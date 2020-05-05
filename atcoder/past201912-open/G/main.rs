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

const MIN: isize = std::isize::MIN;

fn rec(cur: usize, n: usize, class: &mut Vec<usize>, table: &Vec<Vec<Option<isize>>>) -> isize {
    if cur == n {
        let mut score: isize = 0;
        for c in 0..3 {
            let mut temp: Vec<usize> = vec![];
            for i in 0..n {
                if class[i] == c {
                    for &t in temp.iter() {
                        score = score + table[t][i].unwrap();
                    }
                    temp.push(i);
                }
            }
        }
        return score;
    }
    let mut ret = MIN;
    for c in 0..3 {
        class[cur] = c;
        ret = max(rec(cur+1, n, class, table), ret);
    }
    ret
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

    let mut table = vec![vec![None; n]; n];
    for i in 0..n-1 {
        let vs: Vec<isize> = sc.read_vec(n-1-i);
        for j in i+1..n {
            table[i][j] = Some(vs[j-i-1]);
            table[j][i] = Some(vs[j-i-1]);
        }
    }
    let mut class = vec![0; n];
    let ans = rec(0, n, &mut class, &table);
    puts!("{}\n", ans);
    // for r in table {
    //     debug!(r);
    // }
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