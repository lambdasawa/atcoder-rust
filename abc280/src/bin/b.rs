use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        N: usize,
        S: [i128; N],
    }

    let mut A = VecDeque::new();

    let mut total = 0;
    for i in 0..N {
        let a = S[i] - total;
        total += a;
        A.push_back(a);
    }

    println!(
        "{}",
        A.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
