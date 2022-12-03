use proconio::input;

fn f(low: i64, hi: i64, v: i64) -> bool {
    low.min(hi) <= v && v <= low.max(hi)
}

fn main() {
    input! {
        goal: i64,
        wall: i64,
        hammer: i64,
    }

    if f(0, goal, wall) {
        if f(0, hammer, wall) {
            println!("-1");
            return;
        } else {
            println!("{}", (0 - hammer).abs() + (hammer - goal).abs());
            return;
        }
    } else {
        println!("{}", (0 - goal).abs());
        return;
    }
}
