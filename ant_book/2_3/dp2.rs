const N: usize = 4;
const WS: [usize; 4] = [2, 1, 3, 2];
const VS: [usize; 4] = [3, 2, 4, 2];
const W: usize = 5;

use std::cmp;

fn solve() -> usize {
  let mut dp = vec![ vec![0; W+1]; N+1];

  for i in (0..N).rev() {
    for j in 0..W+1 {
      if j < WS[i] {
        dp[i][j] = dp[i + 1][j];
      } else {
        dp[i][j] = cmp::max(dp[i + 1][j], dp[i+1][j-WS[i]] + VS[i]);
      }
    }
  }
  dp[0][W]
}

fn main() {
  println!("{}", solve());
}