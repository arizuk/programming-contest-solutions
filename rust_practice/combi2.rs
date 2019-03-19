fn combi(start: usize, end: usize, idx: usize, res: &mut Vec<usize>, f: &Fn(&Vec<usize>)) {
  for i in start..end {
    res[idx] = i;
    if idx == res.len() - 1 {
      f(res);
    } else {
      combi(i + 1, end, idx + 1, res, f);
    }
  }
}

fn printer(a: &Vec<usize>) {
  println!("{:?}", a);
}


fn main() {
  let n = 5;
  let r = 3;
  let mut a = vec![0; r];
  combi(0, n, 0, &mut a, &printer)
}
