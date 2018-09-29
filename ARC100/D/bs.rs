fn main() {
  let s = [
      (0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
      (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
      (1, 21), (2, 34), (4, 55)
    ];

    // assert_eq!(s.binary_search_by_key(&17, |&(a,b)| b * 2),  Ok(8));
    println!("{:?}", s.binary_search_by_key(&24, |&(a,b)| b * 3));
  // assert_eq!(s.binary_search_by_key(&4, |&(a,b)| b),   Err(7));
  // assert_eq!(s.binary_search_by_key(&100, |&(a,b)| b), Err(13));

  // let r = s.binary_search_by_key(&true, |&(a,b)| b == 8);
  // println!("{:?}", r);

  std::cmp()
}