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

#[derive(Debug)]
struct Bucket {
  seg: Vec<usize>,
  max: usize,
}

impl Bucket {
  fn query(&self, l: usize, r: usize) -> usize {
    let last = min(r+1, self.seg.len());
    let mut ans = self.seg[l];
    for i in l..last {
      ans = max(self.seg[i], ans)
    }
    ans
  }
}

#[derive(Debug)]
struct SegmentTree {
  buckets: Vec<Bucket>
}

impl SegmentTree {
  // [l, r]
  fn query(&self, l: usize, r: usize) -> usize {
    let mut start = 0;
    let mut ans = 0;
    for i in 0..self.buckets.len() {

      let bucket = &self.buckets[i];
      let last = start + bucket.seg.len() - 1;

      if l <= start && last <= r {
        ans = max(ans, bucket.max);
      } else if start <= l && r <= last {
        ans = max(ans, bucket.query(l-start, r-start));
      } else if start <= l && r > last  {
        ans = max(ans, bucket.query(l-start, r-start));
      } else if l < start && r >= start  {
        ans = max(ans, bucket.query(0, r-start));
      }

      start = last + 1;
    }
    ans
  }

  // [l, r]
  fn update(&mut self, k: usize, v: usize) {
    let mut start = 0;
    for i in 0..self.buckets.len() {
      let bucket = &mut self.buckets[i];
      let last = start + bucket.seg.len() - 1;
      if start <= k && k <= last {
        bucket.seg[k-start] = v;
        bucket.max = *bucket.seg.iter().max().unwrap();
      }
      start = last + 1;
    }
  }
}

fn f(l: usize, r: usize, tree: &SegmentTree) -> usize {
    if l > r { return 0 };
    let mlr = tree.query(l-1, r-1);
    debug!(l, r, mlr);
    r-l+1+f(l, mlr-1,tree)+f(mlr+1, r, tree)
}

fn main() {
    input!{
      n: usize,
      q: usize,
      ps: [usize; n],
      ls: [usize; q],
      rs: [usize; q],
    }

    let mut buckets = vec![];
    let mut tmp = vec![];
    for i in 0..n {
        tmp.push(ps[i]);
        if tmp.len()==1000 {
            let bucket = Bucket {
                seg: tmp.clone(),
                max: *tmp.iter().max().unwrap(),
            };
            buckets.push(bucket);
            tmp = vec![];
        }
    }
    if tmp.len() > 0 {
        let bucket = Bucket {
            seg: tmp.clone(),
            max: *tmp.iter().max().unwrap(),
        };
        buckets.push(bucket);
    }

    let tree = SegmentTree { buckets: buckets };
    for i in 0..q {
        let ans = f(ls[i], rs[i], &tree);
        if i != 0 {
            print!(" ");
        }
        print!("{}", ans);
    }
    println!();
}
