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
