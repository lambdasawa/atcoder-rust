use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        D: usize,
        LR: [(usize, usize); D],
    }

    let mut left_to_right = vec![0; N];
    left_to_right[0] = A[0];
    for i in 1..N {
        left_to_right[i] = left_to_right[i - 1].max(A[i]);
    }

    let mut right_to_left = vec![0; N];
    right_to_left[N - 1] = A[N - 1];
    for i in (0..N - 1).rev() {
        right_to_left[i] = right_to_left[i + 1].max(A[i]);
    }

    for (L, R) in LR {
        let li = L - 1;
        let ri = R - 1;

        println!("{}", left_to_right[li - 1].max(right_to_left[ri + 1]));
    }
}
