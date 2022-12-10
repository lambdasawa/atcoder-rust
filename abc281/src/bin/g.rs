use proconio::input;

fn main() {
    input! {
        S: String,
        N: usize,
        mut M: usize,
        NS: [usize; N],
        MS: [usize],
        G: [[usize; N]; N],
        P: [(usize, usize); N],
    }

    println!("{}", S);
}
