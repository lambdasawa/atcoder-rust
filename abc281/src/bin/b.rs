use lazy_static::lazy_static;
use proconio::{input, marker::Bytes};
use regex::bytes::Regex;

fn main() {
    input! {
        s: Bytes,
    }

    lazy_static! {
        static ref R: Regex = Regex::new(r"\A[A-Z][1-9][0-9]{5}[A-Z]\z").unwrap();
    };
    println!("{}", if R.is_match(&s) { "Yes" } else { "No" });
}
