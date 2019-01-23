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

fn main() {
    input!{
      board: [chars; 8],
    }
    let mut board = board;
    for i in 0..64 {
        let x = i/8;
        let y = i%8;
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
                        if board[nx as usize][ny as usize] != 'Q' {
                            board[nx as usize][ny as usize] = '*';
                        }
                        step += 1;
                    }
                }
            }
        }
    }

    for i in 0..64 {
        if board[i/8][i%8] != '.' {
            continue;
        }
        board[i/8][i%8] = 'Q';
        for j in 0..64 {
            if board[j/8][j%8] != '.' {
                continue;
            }
            board[j/8][j%8] = 'Q';
            for k in 0..64 {
                if board[k/8][k%8] != '.' {
                    continue;
                }
                board[k/8][k%8] = 'Q';
                for l in 0..64 {
                    if board[l/8][l%8] != '.' {
                        continue;
                    }
                    board[l/8][l%8] = 'Q';
                    for m in 0..64 {
                        if board[m/8][m%8] != '.' {
                            continue;
                        }
                        board[m/8][m%8] = 'Q';
                        if is_ok(&board) {
                            return print_board(&board);
                        }
                        board[m/8][m%8] = '.';
                    }
                    board[l/8][l%8] = '.';
                }
                board[k/8][k%8] = '.';
            }
            board[j/8][j%8] = '.';
        }
        board[i/8][i%8] = '.';
    }
    println!("{}", "No Answer");
}
