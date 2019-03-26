#[derive(Debug)]
struct Combination {
  n: usize,
  cur: Vec<usize>,
}

impl Combination {
  fn new(n: usize, k: usize) -> Self {
    let cur = vec![0; k];
    Combination { n, cur }
  }
}

impl Iterator for Combination {
  type Item = Vec<usize>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut idx = self.cur.len() - 1;
    let k = self.cur.len();
    loop {
      let rem = k - idx;

      if self.cur[idx] + rem < self.n {
        self.cur[idx] += 1;
        for i in idx + 1..k {
          self.cur[i] = self.cur[i - 1] + 1;
        }
        break;
      }

      if idx == 0 {
        for i in 0..k {
          self.cur[i] = i;
        }
        return None;
      }
      idx -= 1;
    }
    Some(self.cur.clone())
  }
}

fn main() {
  let generator = Combination::new(4, 2);
  for combi in generator.into_iter() {
    println!("combi={:?}", combi);
  }
}
