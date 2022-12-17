use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        W: usize,
        WV: [(usize, usize); N],
    }

    let mut dp = vec![vec![0; W + 1]; N + 1];

    for i in 0..N {
        let (w, v) = WV[i];
        for j in 0..=W {
            if j < w {
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = dp[i][j].max(dp[i][j - w] + v);
            }
        }
    }

    println!("{}", dp[N][W]);
}
