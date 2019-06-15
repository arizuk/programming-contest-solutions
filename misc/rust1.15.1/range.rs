#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        writeln!(&mut std::io::stderr(), concat!("[DEBUG] ", $(stringify!($a), "={:?} "),*), $($a),*);
    }
}
use std::io::Write;

fn main() {
    let a = vec![5, 2, 3, 6];
    let b: Vec<_> = (0..).zip(a.iter()).collect();
    debug!(b);

    // let b: Vec<(&usize,usize)> = a.iter().zip(0..).collect();
    let b: Vec<_> = a.iter().zip(0..).collect();

    use std::collections::HashMap;
    // let b: HashMap<&usize, usize> = a.iter().zip(0..).collect();
    let b: HashMap<_, _> = a.iter().zip(0..).collect();
    for a in a.iter() {
        debug!(a, b[a]);
        if b[a] == 1 {
            println!("b[a] is 1");
        }
    }
}