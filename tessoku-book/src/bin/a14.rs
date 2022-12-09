use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; N],
        D: [usize; N],
    }

    let mut P = vec![0; N * N];
    let mut Q = vec![0; N * N];
    for i in 0..N {
        for j in 0..N {
            P[i * N + j] = A[i] + B[j];
            Q[i * N + j] = C[i] + D[j];
        }
    }

    P.sort();
    Q.sort();

    for p in P {
        if Q.binary_search(&(K - p)).is_ok() {
            println!("{}", "Yes");
            return;
        };
    }

    println!("{}", "No");
}
