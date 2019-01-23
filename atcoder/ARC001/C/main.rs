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

fn is_ok(board: &[Vec<char>]) -> bool {
    for x in 0..8 {
        for y in 0..8 {
            if board[x][y] == 'Q' {
                for dx in -1..2 {
                    for dy in -1..2 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let mut step = 1;
                        loop {
                            let nx = x as i64 + dx*step;
                            let ny = y as i64 + dy*step;
                            if !(nx >= 0 && nx < 8 && ny >= 0 && ny < 8) {
                                break;
                            }
                            if board[nx as usize][ny as usize] == 'Q' {
                                return false
                            }
                            step += 1;
                        }
                    }
                }
            }
        }
    }
    true
}

fn print_board(board: &[Vec<char>]) {
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == '*' {
                // print!("{}", board[i][j]);
                print!(".");
            } else {
                print!("{}", board[i][j]);
            }
        }
        println!();
    }
}

fn dfs(board: &mut [Vec<char>], n: usize, i: usize) -> bool {
    if n == 0 {
        return true;
    }
    if i == 64 {
        return false;
    }

    let x = i/8;
    let y = i%8;

    if board[x][y] == '.' {
        board[x][y] = 'Q';
        if is_ok(board) && dfs(board, n-1, i+1) {
            return true
        }
        board[x][y] = '.';
    }
    dfs(board, n, i+1)
}

fn main() {
    input!{
      board: [chars; 8],
    }
    let mut board = board;
    if dfs(&mut board, 5, 0) {
        print_board(&board);
    } else {
        println!("{}", "No Answer");
    }
}
