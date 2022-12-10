use proconio::input;

fn main() {
    input! {
        N: usize,
        T: usize,
        A: [usize; N],
    }

    let sum: usize = A.iter().sum();
    let sec = T % sum;

    let mut result_i = 0;
    let mut result_sec = 0;
    let mut acc = 0;

    for i in 0..N {
        if sec <= acc + A[i] {
            result_i = i + 1;
            result_sec = sec - acc;
            break;
        } else {
            acc += A[i];
        }
    }

    println!("{} {}", result_i, result_sec);
}
