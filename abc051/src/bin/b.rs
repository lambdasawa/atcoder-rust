use proconio::input;

fn main() {
    input! {
        K: isize,
        S: isize,
    }

    let mut count = 0;

    for x in 0..=K {
        for y in 0..=K {
            let v = S - x - y;
            if 0 <= v && v <= K {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
