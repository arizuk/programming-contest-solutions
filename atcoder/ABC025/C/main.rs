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
use std::io::Write;

fn dfs<F>(cur: usize, board: &mut Vec<Vec<usize>>, f: &F, turn: usize) -> usize
    where F: Fn(&Vec<Vec<usize>>) -> usize
{
    if cur == 9 {
        return f(board)
    }

    let mut scores = vec![];
    for i in 0..9 {
        let x = i/3;
        let y = i%3;
        if board[x][y] == 2 {
            board[x][y] = turn;
            let score = dfs(cur+1, board, f, 1-turn);
            scores.push(score);
            board[x][y] = 2;
        }
    }
    if turn == 0 {
        *scores.iter().max().unwrap()
    } else {
        *scores.iter().min().unwrap()
    }
}

fn main() {
    input!{
      bs: [[usize; 3]; 2],
      cs: [[usize; 2]; 3],
    }

    let f = |board: &Vec<Vec<usize>>| {
        let mut score = 0;
        for i in 0..2 {
            for j in 0..3 {
                if board[i][j] == board[i+1][j] {
                    score += bs[i][j];
                }
            }
        }

        for i in 0..3 {
            for j in 0..2 {
                if board[i][j] == board[i][j+1] {
                    score += cs[i][j];
                }
            }
        }
        score
    };
    let mut board = vec![vec![2; 3]; 3];
    let score = dfs(0, &mut board, &f, 0);


    let mut ttl = 0;
    for i in 0..2 {
        for j in 0..3 {
            ttl += bs[i][j];
        }
    }

    for i in 0..3 {
        for j in 0..2 {
            ttl += cs[i][j];
        }
    }
    println!("{}", score);
    println!("{}", ttl-score);
}
