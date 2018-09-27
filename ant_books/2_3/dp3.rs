const N: usize = 4;
const WS: [usize; 4] = [2, 1, 3, 2];
const VS: [usize; 4] = [3, 2, 4, 2];
const W: usize = 5;

use std::cmp;

fn solve() -> usize {
  let mut dp = vec![ vec![0; W+1]; N+1];

  // dp[i+1][j]: i番目までの品物から、重さの総和がj以下になるように選んだときの価値の総和の最大値
  // dp[0][j] = 0
  // dp[i+1][j] = max(dp[i][j], dp[i][j-w[i]] + v[i])

  for i in 0..N {
    for j in 0..W+1 {
      if j < WS[i] {
        dp[i+1][j] = dp[i][j];
      } else {
        dp[i+1][j] = cmp::max( dp[i][j], dp[i][j-WS[i]] + VS[i] );
      }
    }
  }
  dp[N][W]
}

fn main() {
  println!("{}", solve());
}