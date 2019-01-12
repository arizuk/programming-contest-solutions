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

fn search(i: usize, j: usize, h: usize, w: usize, ss: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, c: char, c1: &mut usize, c2: &mut usize) {
    if seen[i][j] {
        return;
    }

    if ss[i][j] != c {
        return;
    }

    if c == '#' {
        *c1 += 1;
    } else {
        *c2 += 1;
    }

    seen[i][j] = true;

    let i = i as isize;
    let j = j as isize;
    let h = h as isize;
    let w = w as isize;

    let ds = [
        [1,0],
        [-1,0],
        [0,1],
        [0,-1],
    ];

    for d in ds.iter() {
        let dx = d[0];
        let dy = d[1];
        if i + dx < 0 || i + dx >= h {
            continue;
        }

        if j + dy < 0 || j + dy >= w {
            continue;
        }
        search((i+dx) as usize, (j+dy) as usize, h as usize, w as usize, ss, seen, if c == '#'  { '.' } else { '#' }, c1, c2);
    }
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
                let mut c1 = 0;
                let mut c2 = 0;
                if ss[i][j] == '#' {
                    search(i, j, h, w, &ss, &mut seen, '#', &mut c1, &mut c2);
                    ans += c1 * c2;
                }
            }
        }
    }
    println!("{}", ans);
}
