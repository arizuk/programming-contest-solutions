// [l, r]
fn binary_search<F>(mut l: usize, mut r: usize, f: &F) -> Option<usize>
  where F: Fn(usize) -> bool
  {
    assert!(l <= r);
    r += 1;
    let r_bound = r;
    while r > l {
        let m = l + (r - l) / 2;  // avoid overflow
        if f(m) {
          r = m;
        } else {
          if l == m { break; }
          l = m;
        }
    }
    if r == r_bound { None } else { Some(r) }
}

fn main() {
  let f = |m| {
    m*m >= 1000
  };
  let ans = binary_search(1, 1000, &f);
  assert_eq!(Some(32), ans);

  let ans = binary_search(1, 32, &f);
  assert_eq!(Some(32), ans);

  let ans = binary_search(1, 31, &f);
  assert_eq!(None, ans);

  let ans = binary_search(32, 100, &f);
  assert_eq!(Some(32), ans);

  let ans = binary_search(33, 100, &f);
  assert_eq!(Some(33), ans);
}