use proconio::input;

fn main() {
    input! {
        S: String,
    }

    println!(
        "{}",
        S.chars()
            .map(|c| if c == 'v' { 1 } else { 2 })
            .fold(0, |a, b| a + b)
    );
}
