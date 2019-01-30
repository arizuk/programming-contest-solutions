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
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
struct Node {
    v: i64,
    i: usize,
}

#[derive(Debug)]
struct Heap {
    last: usize,
    buf: Vec<Option<Node>>,
}

impl Heap {
    fn new (size: usize) -> Heap {
        let mut s = 1;
        while s < size {
            s *= 2;
        }
        return Heap { buf: vec![None; s], last: 0 }
    }

    fn add(&mut self, v: Node) {
        let mut cur = self.last;
        self.buf[cur] = Some(v);
        self.last += 1;

        loop {
            if cur == 0 { break; }
            let par = (cur-1)/2;

            // let v1 = self.buf[par].as_ref().map(|n| n.v).unwrap_or(0);
            // let v2 = self.buf[cur].as_ref().map(|n| n.v).unwrap_or(0);

            if self.buf[par] > self.buf[cur] {
                break;
            }
            self.buf.swap(par, cur);
            cur = par;
        }
    }

    fn pop(&mut self) -> Option<Node> {
        let peek = std::mem::replace(&mut self.buf[0], None);
        if self.last == 0 {
            return peek
        }

        let last_node = std::mem::replace(&mut self.buf[self.last-1], None);
        self.last -= 1;

        let mut cur = 0;
        self.buf[cur] = last_node;
        loop {
            let left = (cur+1) * 2 - 1;
            let right = (cur+1) * 2;

            if left >= self.buf.len() {
                break;
            }

            if self.buf[left] <= self.buf[cur] && self.buf[right] <= self.buf[cur] {
                break;
            }

            if self.buf[left] > self.buf[right] {
                self.buf.swap(left, cur);
                cur = left;
            } else {
                self.buf.swap(right, cur);
                cur = right;
            }
        }

        peek
    }
}

fn main() {
    let mut heap = Heap::new(10);
    heap.add(Node { v: 1, i: 1 });
    heap.add(Node { v: 5, i: 1 });
    heap.add(Node { v: 2, i: 1 });
    heap.add(Node { v: 3, i: 1 });
    heap.add(Node { v: 10, i: 1 });
    debug!(heap);
    debug!(heap.pop());
    debug!(heap.pop());
    debug!(heap.pop());
    debug!(heap.pop());
    debug!(heap.pop());
    debug!(heap.pop());
    println!("{}", None < Some(Node { v: -1, i: 0 }));
}
