use proconio::input;
use regex::bytes::Regex;

fn main() {
    input! {
        s: Bytes,
    }

    lazy_static! {
        static ref R: Regex = Regex::new(r"\A[A-Z][0-9]{6..}[A-Z]\z").unwrap();
    };
    println!("{}", if R.is_match(&s) { "YES" } else { "NO" });
}
