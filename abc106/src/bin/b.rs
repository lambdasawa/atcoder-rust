use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
    }

    let mut answer = 0;

    for i in 1..=N {
        if i % 2 == 0 {
            continue;
        }

        let mut count = 0;

        for j in 1..=i {
            if i % j == 0 {
                count += 1;
            }
        }

        if count == 8 {
            //            println!("{}", i);
            answer += 1;
        }
    }

    println!("{}", answer);
}
