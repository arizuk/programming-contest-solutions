use std::io::Write;
use std::cmp::min;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}



pub struct Scanner<R> {
    reader: R,
}
impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf: String = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect();
        buf.parse::<T>().ok().expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

fn win_move(i: usize) -> char {
    if i == 0 {
        'P' // P -> R
    } else if i == 1 {
        'S' // S -> P
    } else {
        'R'
    }
}

fn rec(rs: &Vec<Vec<char>>, i: usize, ans: &mut Vec<char>, wins: &mut Vec<bool>) -> bool {
    let mut rps: Vec<usize> = vec![0; 3];
    let mut indices = vec![vec![]; 3];
    for j in 0..rs.len() {
        if wins[j] {
            continue;
        }
        let r = &rs[j];
        let mv = r[ i % r.len() ];
        if mv == 'R' {
            rps[0] += 1;
            indices[0].push(j);
        } else if mv == 'P' {
            rps[1] += 1;
            indices[1].push(j);
        } else { // s
            rps[2] += 1;
            indices[2].push(j);
        }
    }

    let moves: Vec<bool> = rps.iter().map(|&v| v > 0).collect();
    let count: usize = rps.iter().map(|&v| min(1, v)).sum();
    if count == 0 {
        // All win
        return true
    } else if count == 1 {

        for i in 0..3 {
            if rps[i] > 0 {
                let c = win_move(i);
                ans.push(c);
                return true;
            }
        }

    } else if count == 2 {
        if moves[0] && moves[1] { // R & P
            ans.push('P');
            for &idx in indices[0].iter() { wins[idx] = true }
        } else if moves[0] && moves[2] { // R & S
            ans.push('R');
            for &idx in indices[2].iter() { wins[idx] = true }
        } else if moves[1] && moves[2] { // P & S
            ans.push('S');
            for &idx in indices[1].iter() { wins[idx] = true }
        } else {
            unreachable!();
        }
        return rec(rs, i+1, ans, wins);
    } else if count == 3 {
        return false;
    }
    unreachable!();
}

fn main() {
  use std::io::stdin;
  let stdin = stdin();
  let mut scanner = Scanner { reader:stdin.lock() };
  let t: usize = scanner.read();
  for i in 0..t {
      let a: usize = scanner.read();
      let mut rs = vec![];
      for _ in 0..a {
          let r = scanner.chars();
          rs.push(r);
      }

      let mut ans = vec![];
      let mut wins = vec![false; rs.len()];
      if !rec(&rs, 0, &mut ans, &mut wins) {
          println!("Case #{}: {}", i+1, "IMPOSSIBLE");
      } else {
          println!("Case #{}: {}", i+1, ans.iter().collect::<String>());
      }
  }
}