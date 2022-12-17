use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        M: usize,
        S: [String; N],
    }

    let mut count = 0;

    for i in 0..N {
        for j in i..N {
            if i == j {
                continue;
            }

            if S[i]
                .chars()
                .zip(S[j].chars())
                .all(|(ci, cj)| ci == 'o' || cj == 'o')
            {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
