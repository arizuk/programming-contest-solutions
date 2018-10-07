fn main() {
  let aa = [(1,2),(3,4)];
  let mut xs = [0; 2];
  let mut ys = [0; 2];
  for (i, &(x, y)) in aa.iter().enumerate() {
    xs[i] = x;
    ys[i] = y;
  }
  println!("xs={:?} ys={:?}", xs, ys);
}