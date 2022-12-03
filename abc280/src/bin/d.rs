use std::collections::HashMap;

use num::integer::Roots;
use proconio::input;

fn factorize(mut n: usize) -> HashMap<usize, usize> {
    let mut result: HashMap<usize, usize> = HashMap::new();

    for p in 2..=(n.sqrt()) {
        if n % p != 0 {
            continue;
        }

        let mut e = 0;
        while n % p == 0 {
            e += 1;
            n /= p;
        }

        result.insert(p, e);
    }

    if n != 1 {
        result.insert(n, 1);
    }

    return result;
}

fn main() {
    input! {
        K: usize,
    }

    let mut remains = factorize(K);

    for i in 2..=15 {
        for (n, c) in factorize(i) {
            match remains.get(&n) {
                Some(remain) => {
                    if remain <= &c {
                        remains.remove(&n);
                    } else {
                        remains.insert(n, remain - c);
                    }
                }
                None => {}
            }
        }

        if remains.len() == 0 {
            println!("{}", i);
            return;
        }
    }

    println!("{}", K);
}
