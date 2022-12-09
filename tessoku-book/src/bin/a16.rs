use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N-1],
        B: [usize; N-2],
    }

    let mut dp = vec![std::usize::MAX; N];

    dp[0] = 0;
    dp[1] = A[0];
    for i in 2..N {
        dp[i] = (dp[i - 1] + A[i - 1]).min(dp[i - 2] + B[i - 2]);
    }

    println!("{}", dp[N - 1]);
}
