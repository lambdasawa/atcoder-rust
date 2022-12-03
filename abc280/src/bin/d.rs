use std::collections::HashMap;

use num::{integer::Roots, Integer};
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
        mut K: usize,
    }

    let mut k = K;
    for i in 2..=K.sqrt() {
        while k % i == 0 {
            k /= i
        }
    }
    if k != 1 {
        println!("{}", k);
        return;
    }

    for i in 2..=K {
        K /= K.gcd(&i);
        if K == 1 {
            println!("{}", i);
            return;
        }
    }
}
