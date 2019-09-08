fn median(ps: &Vec<usize>) -> usize {
    let n = ps.len();
    if n == 1 {
        return ps[0];
    }

    let mut medians = vec![];
    for i in 0..n-2 {
        let mut vals = vec![ps[i], ps[i+1], ps[i+2]];
        vals.sort();
        medians.push(vals[1]);
    }
    return median(&medians)
}


fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let n:usize = sc.read();
    let x:usize = sc.read();
    let ans: String = sc.read();
    let ps: Vec<usize> = sc.read_vec(2*n-1);

    let mut ps2 = ps.clone();
    ps2.sort();
    for i in 0..2*n-1 {
        if ps2[i] != i+1 {
            return println!("ng ps={:?}", ps);
        }
    }

    let res = median(&ps);
    if res == x {
        println!("{}", "ok");
    } else {
        println!("ng x={} ans={}", x, res);
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

