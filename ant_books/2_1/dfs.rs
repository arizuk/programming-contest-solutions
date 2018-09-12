fn dfs(i: usize, sum: usize, t: usize, a: &[usize]) -> bool {
  if i == a.len() { return sum == t };
  if dfs(i+1, sum, t, a) { return true };
  if dfs(i+1, sum + a[i], t, a) { return true };
  false
}

fn main() {
  let a = [1, 2, 3, 4];
  let t = 7;
  println!("Make {} in {:?}", t, a);
  println!("{}", if dfs(0, 0, t, &a) { "Yes" } else { "No" });
}