#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

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
    use std::cmp::{PartialOrd,Ord};
    use std::ops::{AddAssign, Sub};
    use std::cmp::{min, max};
    #[derive(Debug)]
    pub struct BIT<T> {
        size: usize,
        data: Vec<T>,
    }
    impl<T> BIT<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + Default + PartialOrd + Ord,
    {
        pub fn new(size: usize) -> Self {
            let buf_size = size.next_power_of_two();
            BIT {
                size: size,
                data: vec![T::default(); buf_size + 1],
            }
        }
        pub fn query(&self, i: usize) -> T {
            let mut i = i as i64;
            let mut ret = T::default();
            while i > 0 {
                ret = max(ret, self.data[i as usize]);
                i -= i & -i;
            }
            ret
        }
        pub fn set(&mut self, i: usize, value: T) {
            assert!(i > 0);
            let n = self.data.len() as i64;
            let mut i = i as i64;
            while i <= n - 1 {
                self.data[i as usize] = max(self.data[i as usize], value);
                i += i & -i;
            }
        }
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      n: usize,
      mut whs: [(usize, usize); n],
    }

    let mut bit = ds::BIT::new(1e5 as usize);
    whs.sort_by_key(|v|v.0);

    let mut ans = 0;
    let mut q = vec![];
    for i in 0..n {
        let (w, h) = whs[i];

        if i > 0 && w != whs[i-1].0  {
            for (h, v) in q {
                bit.set(h, v);
            }
            q = vec![];
        }

        let res: usize = bit.query(h-1);
        ans = max(ans, res+1);
        q.push((h, res+1));
    }
    puts!("{}\n", ans);
}
