use std::cmp;

fn solve(s: &[char], t: &[char]) {
  // dp[i][j] = 0..si, 0..tj　に対する共通部分文字列の最大値
  let mut dp = vec![vec![0; t.len()+1]; s.len()+1];

  for i in 0..s.len() {
    for j in 0..t.len() {
      if s[i] == t[j] {
        dp[i+1][j+1] = dp[i][j] + 1;
      } else {
        dp[i+1][j+1] = cmp::max(dp[i+1][j], cmp::max(dp[i][j+1], dp[i][j]));
      }
    }
  }

  println!("   {:?}", t);
  println!("{}: {:?}", ' ', dp[0]);
  for i in 0..s.len() {
    println!("{}: {:?}", s[i], dp[i+1]);
  }
}

fn main() {
  let s: Vec<char> = "abcdef".chars().collect();
  let t: Vec<char> = "bacdfe".chars().collect();
  solve(&s, &t);
}