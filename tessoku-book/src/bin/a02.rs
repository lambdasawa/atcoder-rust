use proconio::input;

fn main() {
    input! {
        N: i64,
        X: i64,
        A: [i64; N],
    }

    println!(
        "{}",
        if A.iter().find(|a| X == **a).is_some() {
            "Yes"
        } else {
            "No"
        }
    );
}
