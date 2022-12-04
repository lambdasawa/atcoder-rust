use proconio::input;

fn main() {
    input! {
        D: usize,
        N: usize,
        LR: [(usize, usize); N],
    }

    let mut accum = vec![0; D + 1];

    for (L, R) in LR {
        accum[L-1] += 1;
        accum[R] -= 1;
    }

    let mut sum = 0;
    for i in 0..D {
        sum += accum[i];
        println!("{}", sum);
    }
}
