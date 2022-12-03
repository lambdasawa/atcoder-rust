use proconio::input;

fn main() {
    input! {
        N: i64,
        K: i64,
    }

    let mut c = 0;
    for i in 1..=N {
        for j in 1..=N {
            let k = K - i - j;
            if 1 <= k && k <= N {
                c += 1;
            }
        }
    }

    println!("{}", c);
}
