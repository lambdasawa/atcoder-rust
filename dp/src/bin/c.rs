use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        ABC: [(usize, usize, usize); N],
    }

    let mut dp = vec![vec![0; 4]; N + 1];

    let (a, b, c) = ABC[0];
    dp[0][1] = a;
    dp[0][2] = b;
    dp[0][3] = c;

    for i in 1..N {
        let (a, b, c) = ABC[i];

        dp[i][1] = dp[i - 1][2].max(dp[i - 1][3]) + a;
        dp[i][2] = dp[i - 1][1].max(dp[i - 1][3]) + b;
        dp[i][3] = dp[i - 1][1].max(dp[i - 1][2]) + c;
    }

    println!("{}", dp[N - 1].iter().max().unwrap());
}
