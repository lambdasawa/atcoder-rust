use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N],
    }

    let mut l = 0;
    let mut r = N - 1;
    while l <= r {
        let m = (l + r) / 2;
        let x = A[m];
        if x < X {
            l = m + 1;
        }
        if x > X {
            r = m - 1;
        }
        if x == X {
            println!("{}", m + 1);
            return;
        }
    }
}
