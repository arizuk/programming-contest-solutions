// [l, r]
fn binary_search<F>(mut l: i64, mut r: i64, f: &F) -> Option<i64>
  where F: Fn(i64) -> bool
  {
    assert!(l <= r);
    l -= 1;
    r += 1;
    let r_bound = r;
    while r - l > 1 {
        let m = l + (r - l) / 2;  // avoid overflow
        if f(m) {
          r = m;
        } else {
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
}