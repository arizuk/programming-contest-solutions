// 2d cumulative sum

fn get_sum(sxy: &Vec<Vec<usize>>, x: usize, lx: usize, y: usize, ly: usize) -> usize {
  sxy[x+lx][y+ly] + sxy[x][y] - sxy[x+lx][y] - sxy[x][y+ly]
}

fn main() {
  let xys = [
    [2,1,1],
    [1,2,1],
    [1,1,2],
  ];

  let n = xys.len();
  let mut sxy = vec![vec![0; n+1]; n+1];

  for x in 0..n {
    for y in 0..n {
      sxy[x+1][y+1] = sxy[x+1][y] + sxy[x][y+1] + xys[x][y] - sxy[x][y];
    }
  }

  for x in (0..n).rev() {
    for y in 0..n {
      print!("{:3}", xys[x][y]);
    }
    println!();
  }
  println!("-----------");

  for x in (0..n+1).rev() {
    for y in 0..n+1 {
      print!("{:3}", sxy[x][y]);
    }
    println!();
  }

  assert_eq!(2, get_sum(&sxy, 2, 1, 2, 1));
  assert_eq!(3, get_sum(&sxy, 1, 2, 2, 1));
  assert_eq!(6, get_sum(&sxy, 1, 2, 1, 2));
  assert_eq!(8, get_sum(&sxy, 1, 2, 0, 3));
  assert_eq!(5, get_sum(&sxy, 0, 2, 1, 2));
}