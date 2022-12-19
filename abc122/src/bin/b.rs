use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: String,
    }

    let mut count = 0;
    let mut max_count = 0;

    for c in S.chars().into_iter() {
        match c {
            'A'| 'C'| 'G'| 'T' => {
                count += 1;
            },
            _ => {
                max_count = max_count.max(count);
                count = 0;
            }
        }
    }
    max_count = max_count.max(count);

    println!("{}", max_count);
}
