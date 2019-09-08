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

pub mod ds {
    use std::cmp::PartialOrd;
    use std::ops::{AddAssign, Sub};
    #[derive(Debug)]
    pub struct BIT<T> {
        size: usize,
        data: Vec<T>,
    }
    impl<T> BIT<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + Default + PartialOrd,
    {
        pub fn new(size: usize) -> Self {
            let buf_size = size.next_power_of_two();
            BIT {
                size: size,
                data: vec![T::default(); buf_size + 1],
            }
        }
        #[doc = " [l, r) l,r: 1-indexed"]
        pub fn sum_between(&self, l: usize, r: usize) -> T {
            self.sum(r - 1) - self.sum(l - 1)
        }
        #[doc = " i: 1-indexed but returns 0 if i=0 is given."]
        pub fn sum(&self, i: usize) -> T {
            let mut i = i as i64;
            let mut ret = T::default();
            while i > 0 {
                ret += self.data[i as usize];
                i -= i & -i;
            }
            ret
        }
        #[doc = " i: 1-indexed"]
        pub fn add(&mut self, i: usize, value: T) {
            assert!(i > 0);
            let n = self.data.len() as i64;
            let mut i = i as i64;
            while i <= n - 1 {
                self.data[i as usize] += value;
                i += i & -i;
            }
        }
        pub fn lower_bound(&self, value: T) -> usize {
            let mut r = self.size + 1;
            let mut l = 1;
            while r != l {
                let mid = (r + l) / 2;
                if self.sum(mid) >= value {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            r
        }
    }
}

pub fn lower_bound<T: Ord>(a: &Vec<T>, x: &T) -> usize {
    use std::cmp::Ordering;
    let mut l = 0;
    let mut r = a.len();
    while l != r {
        let m = l + (r - l) / 2;
        match a[m].cmp(x) {
            Ordering::Less => l = m + 1,
            Ordering::Equal | Ordering::Greater => r = m,
        }
    }
    l
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    let stdin = std::io::stdin();
    let mut sc = Scanner { reader: stdin.lock() };

    let n: usize = sc.read();
    let nums = 2usize.pow(n as u32);
    let mut s: Vec<usize> = sc.read_vec(nums);

    let mut comp = s.clone();
    comp.sort();
    comp.dedup();

    let mut bit = ds::BIT::new(nums);

    for &v in s.iter() {
        let idx = comp.binary_search(&v).unwrap();
        bit.add(idx+1, 1 as i64);
    }
    s.sort();

    let mut used = vec![];
    {
        let v = s[s.len()-1];
        let idx = comp.binary_search(&v).unwrap();
        bit.add(idx+1, -1);
        used.push(v);
    }

    while used.len() < nums {
        let mut to_insert = vec![];

        for &v in used.iter() {
            let idx = comp.binary_search(&v).unwrap();
            if idx == 0 {
                return puts!("{}\n", "No");
            }
            let cnt = bit.sum((idx-1)+1);
            if cnt == 0 {
                return puts!("{}\n", "No");
            }
            let used_pos = bit.lower_bound(cnt);
            bit.add(used_pos, -1);
            to_insert.push( comp[used_pos-1] );
        }

        for v in to_insert {
            used.push(v);
        }
    }
    puts!("{}\n", "Yes");
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