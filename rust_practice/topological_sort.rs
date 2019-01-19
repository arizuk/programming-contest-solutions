fn main() {
  let inputs = [
    (5,3),
    (2,3),
    (2,4),
    (5,2),
    (5,1),
    (1,4),
    (4,3),
    (1,3)
  ];

  let mut edges = vec![vec![]; 5];
  let mut h = vec![0; 5];
  for &(x, y) in inputs.iter() {
    edges[x-1].push(y-1);
    h[y-1] += 1;
  }
  println!("edges={:?}", edges);

  let mut st = Vec::new();
  for i in 0..edges.len() {
    if h[i] == 0 {
      st.push(i);
    }
  }

  let mut ans = vec![];
  while let Some(i) = st.pop() {
    ans.push(i);
    for &t in edges[i].iter() {
      h[t] -= 1;
      if h[t] == 0 {
        st.push(t);
      }
    }
  }
  for i in ans.iter() {
    println!("{}", i+1);
  }
}