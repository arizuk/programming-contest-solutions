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
use std::io::Write;

struct RMQ {
    blocks: Vec<usize>,
    data: Vec<usize>,
}


impl RMQ {
    fn new(data: Vec<usize>) -> Self {
        let block_size = (data.len() as f64).sqrt() as usize + 1;
        let mut blocks = vec![0; block_size];
        for i in 0..data.len() {
            let block_index = i/block_size;
            blocks[block_index] = max(data[i], blocks[block_index]);
        }
        RMQ {
            blocks: blocks,
            data: data,
        }
    }

    fn update(&mut self, i:usize, v: usize) {
        self.data[i] = v;
        let block_index = i/self.blocks.len();
        self.blocks[block_index] = max(self.blocks[block_index], v);
    }

    // [l,r]
    fn query(&self, l: usize, r:usize) -> usize {
        assert!(l <= r);
        let block_size = self.blocks.len();
        let lb = l/block_size;
        let rb = r/block_size;
        let mut cur = lb;
        let mut ans = 0;
        while cur <= rb {
            if cur == lb || cur == rb {
                let mut ll = cur * block_size;
                let mut rr = (cur+1) * block_size - 1;
                if cur == lb { ll = l; }
                if cur == rb { rr = r; }

                for i in ll..rr+1 {
                    ans = max(ans, self.data[i]);
                }
            } else {
                ans = max(ans, self.blocks[cur]);
            }
            cur += 1;
        }
        ans
    }
}


fn main() {
    input!{
      n: usize,
      hs: [usize; n],
      aa: [usize; n],
    }
    let mut rmq = RMQ::new(vec![0; n+1]);

    for i in 0..n {
        let h = hs[i];
        let max_v = rmq.query(0, h-1);
        rmq.update(h, max_v+aa[i]);
    }
    let ans = rmq.query(0, n);
    println!("{}", ans);
}
