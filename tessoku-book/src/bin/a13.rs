use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [isize; N],
    }

    let mut count = 0;

    let mut r = 0;

    for l in 0..N {
        while r < N && A[r] - A[l] <= K as isize {
            r += 1;
        }

        count += r - l - 1;
    }

    println!("{}", count);
}
