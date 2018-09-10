use std::collections::BTreeSet;

fn main() {
  let a: [i32; 3] = [1, 2, 3];
  // let a = [1, 2, 3];

  // この書き方だと &i32 -> i32になるらしい
  // let b: Vec<i32> = a.iter().map(|&x| x).collect();
  let b: Vec<_> = a.iter().map(|x| x * 2).collect();
  // let b: Vec<&i32> = a.iter().collect();

  // let b = a.to_vec();

  // cloneすれば動く模様
  // let b = a.iter().map(|x| x.clone()).collect::<Vec<i32>>();

  println!("{:?}", b);

  let aa = [5, 2, 3, 10, 3, 2];
  let set: BTreeSet<_> = aa.iter().collect();
  // let set = aa.iter().collect::<BTreeSet<_>>();
  println!("{:?} len={}", set, set.len());
}