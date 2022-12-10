use proconio::input;

fn main() {
    input! {
        R1: isize,
        C1: isize,
        R2: isize,
        C2: isize,
    }

    if (R1, C1) == (R2, C2) {
        println!("{}", 0);
        return;
    }

    // 斜め移動
    if (R1 - R2).abs() == (C1 - C2).abs() {
        println!("{}", 1);
        return;
    }

    // キング的な移動
    if (R1 - R2).abs() + (C1 - C2).abs() <= 3 {
        println!("{}", 1);
        return;
    }

    // キング的な移動 *2
    if (R1 - R2).abs() + (C1 - C2).abs() <= 3 + 2 {
        println!("{}", 2);
        return;
    }

    // 2回斜め移動
    if (R1 - R2).abs() % 2 == (C1 - C2).abs() % 2 {
        println!("{}", 2);
        return;
    }

    // 斜め + キング的な移動
    if ((R1 - R2).abs() - (C1 - C2).abs()).abs() <= 3 {
        println!("{}", 2);
        return;
    }

    println!("{}", 3);
    return;
}
