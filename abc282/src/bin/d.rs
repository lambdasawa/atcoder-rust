use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: String,
        C: Chars,
        N: usize,
        mut M: usize,
        NS: [usize; N],
        MS: [usize],
        G: [[usize; N]; N],
        P: [(usize, usize); N],
    }

    // 辺が二個

    println!("{}", S);
}
