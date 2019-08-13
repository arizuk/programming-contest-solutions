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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }

    input!{
      h: usize,
      w: usize,
      mut aa: [chars; h],
    }

    let mut x = 0;
    let mut y = 0;
    while (x, y) != (h-1, w-1) {
        aa[x][y] = '.';
        let dirs = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
        ];
        let mut nexts = vec![];
        for &(dx, dy) in dirs.iter() {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx >= 0 && nx < h as i64 && ny >= 0 && ny < w as i64 {
                let nx = nx as usize;
                let ny = ny as usize;
                if aa[nx][ny] == '#' {
                    if dx == -1 || dy == -1 {
                        return puts!("{}\n", "Impossible");
                    }
                    nexts.push((nx, ny));
                }
            }
        }
        if nexts.len() != 1 {
            return puts!("{}\n", "Impossible");
        }
        x = nexts[0].0;
        y = nexts[0].1;
        // for a in aa.iter() {
        //     puts!("{}\n", a.iter().map(|&c| c).collect::<String>());
        // }
        // puts!("{}\n", "--------------------------");
    }

    aa[0][0] = '.';
    aa[h-1][w-1] = '.';

    for i in 0..h {
        for j in 0..w {
            if aa[i][j] == '#' {
                return puts!("{}\n", "Impossible");
            }
        }
    }

    puts!("{}\n", "Possible");
}
