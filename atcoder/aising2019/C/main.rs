#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

#[allow(unused_imports)]
use std::io::Write;

fn search(i: usize, j: usize, ss: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, c: char) -> (usize, usize) {
    let mut ans = (0, 0);

    if seen[i][j] || ss[i][j] != c {
        return ans
    }

    if c == '#' {
        ans.0 += 1;
    } else {
        ans.1 += 1;
    }

    seen[i][j] = true;

    let ds = [
        [1,0],
        [-1,0],
        [0,1],
        [0,-1],
    ];

    for d in ds.iter() {
        let nx = i as i64 + d[0];
        let ny = j as i64 + d[1];
        if nx < 0 || nx >= ss.len() as _ {
            continue;
        }

        if ny < 0 || ny >= ss[0].len() as _ {
            continue;
        }

        let nc = if c == '#'  { '.' } else { '#' };
        let ret = search(nx as _, ny as _, ss, seen, nc);
        ans.0 += ret.0;
        ans.1 += ret.1;
    }
    ans
}

fn main() {
    input!{
      h: usize,
      w: usize,
      ss: [chars; h],
    }
    let ss = ss;


    let mut seen = vec![vec![false; w]; h];
    let mut ans = 0usize;
    for i in 0..h {
        for j in 0..w {
            if !seen[i][j] {
                if ss[i][j] == '#' {
                    let (c1, c2) = search(i, j, &ss, &mut seen, '#');
                    ans += c1 * c2;
                }
            }
        }
    }
    println!("{}", ans);
}
