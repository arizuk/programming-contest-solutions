fn solve(n: usize, m: usize, ks: &[usize]) {
  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        for l in 0..n {
          if ks[i] + ks[j] + ks[k] + ks[l] == m {
            println!("Yes");
            return;
          }
        }
      }
    }
  }
  println!("No");
}

fn main() {
  solve(3, 10, &[1,3,5]);
}