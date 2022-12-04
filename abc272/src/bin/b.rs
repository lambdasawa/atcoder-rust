use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        X: [[usize]; M],
    }

    let mut set: HashSet<(usize, usize)> = HashSet::new();

    for members in X {
        for a in members.clone() {
            for b in members.clone() {
                set.insert((a, b));
                set.insert((b, a));
            }
        }
    }

    let mut ok = true;
    for a in 1..=N {
        for b in 1..=N {
            if !set.contains(&(a, b)) {
                ok = false;
            }
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
