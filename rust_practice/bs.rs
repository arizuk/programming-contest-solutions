fn bs(l: usize, r: usize, v: usize) -> usize {
  assert!(l <= r);
  let mut l = l;
  let mut r = r;
  l -= 1;
  while r - l > 1 {
      let m = l + (r - l) / 2;  // avoid overflow
      if m >= v {
        r = m;
      } else {
        l = m;
      }
  }
  println!("{}", r);
  r
}

fn main() {
  assert_eq!(3, bs(3, 4, 0));
  assert_eq!(4, bs(3, 4, 4));
  assert_eq!(3, bs(3, 4, 3));
  assert_eq!(11, bs(1, 10+1, 12));
  assert_eq!(11, bs(1, 10+1, 11));
  assert_eq!(1, bs(1, 10+1, 0));
  assert_eq!(3, bs(1, 10+1, 3));
  assert_eq!(10, bs(1, 10+1, 10));
}