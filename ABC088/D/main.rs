#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), "={:?} "),*), $($a),*);
    }
}

#[allow(unused_imports)]
use std::cmp::{min, max};

fn search(ss: &Vec<Vec<char>>, i: i64, j: i64, scores: &mut Vec<Vec<i64>>, score: i64) {
    let h = ss.len() as i64;
    let w = ss[0].len() as i64;
    let n_score = score + 1;

    for &(dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
        let nx = i + dx;
        let ny = j + dy;
        if nx < 0 || nx >= h { continue };
        if ny < 0 || ny >= w { continue };
        let nx = nx as usize;
        let ny = ny as usize;
        if ss[nx][ny] == '#' { continue };

        if n_score >= scores[nx][ny] && scores[nx][ny] != -1 {
            continue;
        }
        scores[nx][ny] = n_score;
        search(ss, nx as i64, ny as i64, scores, n_score);
    }
}


fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let hws = buf.split_whitespace().map(|v| v.parse().unwrap()).collect::<Vec<usize>>();

    let h = hws[0];
    let w = hws[1];

    let mut ss = vec![];
    for _ in 0..h {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let buf = buf.trim_right();
        let row = buf.chars().collect::<Vec<char>>();
        ss.push(row);
    }

    let mut scores = vec![vec![-1i64; w]; h];
    scores[0][0] = 0;
    search(&ss, 0, 0, &mut scores, 0);

    let score = scores[h-1][w-1];
    let mut cnt = 0;
    for row in ss {
        for c in row {
            if c == '.' {
                cnt += 1;
            }
        }
    }
    if score == -1 {
        println!("{}", -1);
    } else {
        println!("{}", cnt-score-1);
    }
}
