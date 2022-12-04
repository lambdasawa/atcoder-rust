use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
        LR: [(usize, usize); Q],
    }

    let mut accum = vec![0; N + 1];

    for i in 1..=N {
        accum[i] = accum[i - 1] + A[i - 1];
    }

    // println!("{:?}", accum);

    for (li, ri) in LR {
        let l = accum[li - 1];
        let r = accum[ri];
        println!("{}", r - l);
    }
}
