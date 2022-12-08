use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }

    let mut l = 1;
    let mut r = 10usize.pow(9);
    while l < r {
        let m = (l + r) / 2;

        let k = sum(&A, m);

        if k >= K {
            r = m;
        } else {
            l = m + 1;
        }
    }

    println!("{}", l);
}

fn sum(A: &Vec<usize>, m: usize) -> usize {
    A.iter().map(|n| m / n).sum::<usize>()
}
