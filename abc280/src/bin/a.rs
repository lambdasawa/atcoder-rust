use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        N: usize,
        S: [usize; N],
    }

    let mut A = VecDeque::new();

    let mut sum = S[0];
    for i in 1..N {
        a = S[i] - sum;
        sum += S[i];
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
