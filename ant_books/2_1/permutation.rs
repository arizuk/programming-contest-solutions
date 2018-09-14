fn permutation(pos: u32, n: u32, used: &mut [bool], perm: &mut [u32]) {
  if pos == n {
    println!("{:?}", perm);
    return;
  }

  for i in 0..n as usize {
    if used[i] == false {
      used[i] = true;
      perm[pos as usize] = i as u32;
      permutation(pos+1, n, used, perm);
      used[i] = false;
    }
  }
}

fn main() {
  let mut used = [false; 3];
  let mut perm = [0; 3];
  permutation(0, 3, &mut used, &mut perm)
}