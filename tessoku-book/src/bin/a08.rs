use proconio::input;

fn main() {
    input! {
        H: usize,
        W: usize,
        X: [[usize; W]; H],
        Q: usize,
        ABCD: [(usize, usize, usize, usize); Q],
    }

    let mut accum = vec![vec![0; W + 1]; H + 1];

    for h in 1..=H {
        for w in 1..=W {
            accum[h][w] = X[h - 1][w - 1] + accum[h - 1][w] + accum[h][w - 1] - accum[h - 1][w - 1];
        }
    }

    for (ly, lx, hy, hx) in ABCD {
        println!(
            "{}",
            accum[hy][hx] - accum[hy][lx - 1] - accum[ly - 1][hx] + accum[ly - 1][lx - 1]
        );
    }
}

fn print_table(table: &Vec<Vec<usize>>) {
    for line in table {
        for n in line {
            print!("{:3} ", n);
        }
        println!("")
    }
}
