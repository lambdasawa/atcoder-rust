use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        K: usize,
        H: [isize; N],
    }

    let H = [vec![0], H].concat();
    let mut dp = vec![std::isize::MAX; N + 1];

    dp[0] = 0;
    dp[1] = 0;
    dp[2] = (H[2] - H[1]).abs();
    for i in 2..=N {
        for j in 1..=K {
            if i <= j {
                break;
            }

            dp[i] = dp[i].min(dp[i - j] + (H[i] - H[i - j]).abs());
        }
    }

    println!("{}", dp[N]);
}
