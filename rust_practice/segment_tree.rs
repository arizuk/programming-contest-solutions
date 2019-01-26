pub struct Scanner<R> {
    reader: R,
}
impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

#[derive(Debug)]
struct Bucket {
  seg: Vec<i64>,
  min: i64,
}

impl Bucket {
  fn query(&self, l: usize, r: usize) -> i64 {
    let last = min(r+1, self.seg.len());
    let mut ans = self.seg[l];
    for i in l..last {
      ans = min(self.seg[i], ans)
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
  fn query(&self, l: usize, r: usize) -> i64 {
    let mut start = 0;
    let mut ans = 1 << 32;
    for i in 0..self.buckets.len() {

      let bucket = &self.buckets[i];
      let last = start + bucket.seg.len() - 1;

      if l <= start && last <= r {
        ans = min(ans, bucket.min);
      } else if start <= l && r <= last {
        ans = min(ans, bucket.query(l-start, r-start));
      } else if start <= l && r > last  {
        ans = min(ans, bucket.query(l-start, r-start));
      } else if l < start && r >= start  {
        ans = min(ans, bucket.query(0, r-start));
      }

      start = last + 1;
    }
    ans
  }

  // [l, r]
  fn update(&mut self, k: usize, v: i64) {
    let mut start = 0;
    for i in 0..self.buckets.len() {
      let bucket = &mut self.buckets[i];
      let last = start + bucket.seg.len() - 1;
      if start <= k && k <= last {
        bucket.seg[k-start] = v;
        bucket.min = *bucket.seg.iter().min().unwrap();
      }
      start = last + 1;
    }
  }
}

use std::cmp::min;

fn main() {
  let io = std::io::stdin();
  let mut sc = Scanner { reader: io.lock() };

  let n: usize = sc.read();
  let q: usize = sc.read();
  let mut tmp: Vec<i64> = vec![];
  let mut buckets = vec![];
  for _ in 0..n {
    tmp.push(sc.read());
    if tmp.len() == 2 {
      let bucket = Bucket {
        seg: tmp.clone(),
        min: *tmp.iter().min().unwrap(),
      };
      buckets.push(bucket);
      tmp = vec![];
    }
  }
  if tmp.len() > 0 {
    let bucket = Bucket {
      seg: tmp.clone(),
      min: *tmp.iter().min().unwrap(),
    };
    buckets.push(bucket);
  }
  let mut tree = SegmentTree { buckets: buckets };

  for _ in 0..q {
    let a: i64 = sc.read();
    let b: usize = sc.read();
    let c: i64 = sc.read();
    if a == 1 {
      println!("{}", tree.query((b-1) as _, (c-1) as _));
    } else {
      tree.update(b-1, c);
    }
  }
}