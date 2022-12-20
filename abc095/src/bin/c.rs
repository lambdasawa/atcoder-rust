use proconio::{input, marker::Chars};

fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        X: usize,
        Y: usize,
    }

    let mut min_cost = std::usize::MAX;

    for i in 0..=(X.max(Y)) {
        let c_cost = C * 2 * i;
        let a_cost = if X >= i { A * (X - i) } else { 0 };
        let b_cost = if Y >= i { B * (Y - i) } else { 0 };

        min_cost = min_cost.min(c_cost + a_cost + b_cost);
    }

    println!("{}", min_cost);
}
