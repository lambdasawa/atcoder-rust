use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        H: [isize; N],
    }

    let H = [vec![0], H].concat();

    let mut dp = vec![std::isize::MAX; N + 1];
    dp[1] = 0;
    dp[2] = (H[2] - H[1]).abs();
    for i in 3..=N {
        let cost1 = dp[i - 1] + (H[i] - H[i - 1]).abs();
        let cost2 = dp[i - 2] + (H[i] - H[i - 2]).abs();
        dp[i] = cost1.min(cost2);
    }

    println!("{}", dp[N]);
}
