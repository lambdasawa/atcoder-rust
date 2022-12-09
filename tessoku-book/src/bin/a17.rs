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

    let mut route = Vec::with_capacity(N + 9);
    let mut room = N - 1;
    loop {
        if room == 0 {
            break;
        }

        if room >= 1 && dp[room] == dp[room - 1] + A[room - 1] {
            route.push(room + 1);
            room -= 1;
        }

        if room >= 2 && dp[room] == dp[room - 2] + B[room - 2] {
            route.push(room + 1);
            room -= 2;
        }
    }
    route.push(1);

    println!("{}", route.len());
    println!(
        "{}",
        route
            .iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
