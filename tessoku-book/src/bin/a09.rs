use proconio::input;

fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        ABCD: [(usize,usize,usize,usize); N],
    }

    let mut accum = vec![vec![0isize; W + 9]; H + 9];

    for (low_h, low_w, high_h, high_w) in ABCD {
        accum[low_h][low_w] += 1;
        accum[low_h][high_w + 1] -= 1;
        accum[high_h + 1][low_w] -= 1;
        accum[high_h + 1][high_w + 1] += 1;
    }

    for h in 1..=H {
        for w in 1..=W {
            accum[h][w] = accum[h][w - 1] + accum[h][w];
        }
    }
    for h in 1..=H {
        for w in 1..=W {
            accum[h][w] = accum[h - 1][w] + accum[h][w];
        }
    }

    for h in 1..=H {
        println!(
            "{}",
            accum[h][1..=W]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

fn print_table(table: &Vec<Vec<isize>>) {
    for line in table {
        for n in line {
            print!("{:4} ", n);
        }
        println!("")
    }
}
