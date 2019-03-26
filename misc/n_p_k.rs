fn perm(i: usize, k: usize, cur: usize, n: usize, res: &mut Vec<usize>) {
  for j in cur..n {
    res[i] = j;
    if i == k - 1 {
      let mut perm_res = vec![-1; k];
      let print_fn = |a: &Vec<isize>| {
        for &idx in a.iter() {
          print!("{} ", res[idx as usize]);
        }
        println!();
      };
      _perm(&mut perm_res, 0, k, &print_fn);
    } else {
      perm(i + 1, k, j + 1, n, res);
    }
  }
}

fn _perm(res: &mut Vec<isize>, i: usize, n: usize, p: &Fn(&Vec<isize>)) {
  if i == res.len() {
    return p(res);
  }

  for idx in 0..n {
    if res[idx] == -1 {
      res[idx] = i as isize;
      _perm(res, i + 1, n, p);
      res[idx] = -1;
    }
  }
}

fn main() {
  let n = 5;
  let k = 3;
  let mut res = vec![0; k];
  println!("Prints {}P{} permutation.", n, k);
  perm(0, k, 0, n, &mut res);
}
