use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [[usize]; N],
        ST: [(usize, usize); Q],
    }

    for (S, T) in ST {
        println!("{}", A[S - 1][T - 1])
    }
}
