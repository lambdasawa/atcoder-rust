use proconio::input;

fn main() {
    input! {
        S: String,
        T: String,
    }

    let s_chars: Vec<char> = S.chars().collect();
    let t_chars: Vec<char> = T.chars().collect();

    let mut found = false;

    for i in 0..S.len() {
        let s = s_chars[i];
        let t = t_chars[i];

        if s != t {
            found = true;
            println!("{}", i + 1);
            break;
        }
    }

    if !found {
        println!("{}", T.len());
    }
}
