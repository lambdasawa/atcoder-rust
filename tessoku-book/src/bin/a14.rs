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
        if binary_search(&Q, K - p).is_some() {
            println!("{}", "Yes");
            return;
        };
    }

    println!("{}", "No");
}

fn binary_search(A: &Vec<usize>, x: usize) -> Option<usize> {
    let len = A.len();

    let mut l = 0;

    let mut r = len - 1;

    while l <= r {
        let m = (l + r) / 2;
        let a = A[m];
        if a < x {
            if m >= len {
                return None;
            }
            l = m + 1;
        }
        if a > x {
            if m <= 0 {
                return None;
            }
            r = m - 1;
        }
        if a == x {
            return Some(m);
        }
    }

    return None;
}
