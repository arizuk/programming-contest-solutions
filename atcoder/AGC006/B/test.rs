fn dump(a: &Vec<usize>, ans: &mut Vec<usize>) {
    if a.len() == 1 {
        ans.push(a[0]);
        print!("{} ", a[0]);
        return println!();
        return;
    }

    for &a in a.iter() {
        print!("{} ", a);
    }

    print!(" => ");

    let mut medians = vec![];
    for i in 0..a.len()-2 {
        let mut vals = vec![a[i], a[i+1], a[i+2]];
        vals.sort();
        medians.push(vals[1]);
    }
    dump(&medians, ans);
}

fn rec(i: usize, n: usize, tmp: &mut Vec<usize>, ans: &mut Vec<usize>) {
    if i == n {
        dump(tmp, ans);
        return;
    }

    for j in 0..n {
        if !tmp.contains(&(j+1)) {
            tmp.push(j+1);
            rec(i+1, n, tmp, ans);
            tmp.pop();
        }
    }
}

fn main() {
    let mut tmp = vec![];
    let n = 3;
    let mut ans = vec![];
    rec(0, 2*n-1, &mut tmp, &mut ans);

    ans.sort();
    ans.dedup();
    println!("ans={:?}", ans);
}