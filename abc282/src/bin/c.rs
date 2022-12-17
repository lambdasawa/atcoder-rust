use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        S: String,
    }

    let mut quoting = false;

    let chars = S.chars().collect::<Vec<char>>();
    let mut answer = Vec::with_capacity(N);

    for i in 0..N {
        if chars[i] == '"' {
            quoting = !quoting;
            answer.push('"');
        } else {
            if chars[i] == ',' && !quoting {
                answer.push('.')
            } else {
                answer.push(chars[i]);
            }
        }
    }

    let a: String = answer.into_iter().collect();
    println!("{}", a);
}
