fn binaryserach(mut l: usize, mut r: usize, f: &Fn(usize) -> bool) -> usize {
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if f(m) {
            println!("{:>2} -  {}  <- {}", l, m, r);
            r = m;
        } else {
            println!("{:>2} -> {}   - {}", l, m, r);
            l = m;
        }
    }
    r
}

fn main() {
  let r = 99;
  let l = 0;

  let mut s = vec![0; 100];
  for i in 0..s.len() {
    s[i] = i * 2;
  }
  let res = binaryserach(l, r, &|m| {
    s[m] >= 50
  });
  println!("i={0} v[i]={1}", res, s[res]);
}