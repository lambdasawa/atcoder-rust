use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
    }

    let s = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    println!("{}", s[0..N].to_string());
}
