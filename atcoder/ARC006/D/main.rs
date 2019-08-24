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

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
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
use std::io::{stdout, stdin, BufWriter, Write};


fn bfs(cur: (usize, usize), cs: &Vec<Vec<char>>, h: usize, w: usize, seen: &mut Vec<Vec<bool>>) -> usize {
    use std::collections::VecDeque;
    let mut count = 0;
    let mut q = VecDeque::new();
    q.push_back(cur);

    let lower = cur;
    let mut upper = cur;

    while q.len() > 0 {
        let (x, y) = q.pop_front().unwrap();
        let dirs = vec![
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        for (dx, dy) in dirs {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if !(nx >= 0 && nx < h as i64 && ny >= 0 && ny < w as i64) {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if seen[nx][ny] {
                continue;
            }
            seen[nx][ny] = true;
            if cs[nx][ny] == 'o' {
                count += 1;
                q.push_back((nx, ny));
                upper = max(upper, (nx, ny));
            }
        }
    }
    let size = upper.0 - lower.0 + 1;
    assert!(size % 5 == 0);
    let size = size/5;
    assert!(count % (size*size) == 0);
    count / (size * size)
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      h: usize,
      w: usize,
      cs: [chars; h],
    }
    let mut seen = vec![vec![false; w]; h];
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for i in 0..h {
        for j in 0..w {
            if cs[i][j] == 'o' && !seen[i][j] {
                let count = bfs((i, j), &cs, h, w, &mut seen);
                match count {
                    12 => a += 1,
                    16 => b += 1,
                    11 => c += 1,
                    _ => unreachable!(),
                }
            }
        }
    }
    puts!("{} {} {}\n", a, b, c);
}
