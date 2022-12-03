use proconio::input;

fn main() {
    input! {
        mut N: u64,
    }

    for i in (0..10).rev() {
        print!("{}", if N / 2u64.pow(i) >= 1 { 1 } else { 0 });
        N = N % 2u64.pow(i);
    }
}
