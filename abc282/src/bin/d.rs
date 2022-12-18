use std::collections::HashMap;

use proconio::{input, marker::Chars};

const COLOR_NONE: usize = 0;
const COLOR_RED: usize = 1;
const COLOR_BLUE: usize = 2;

struct Solver {
    colors: Vec<usize>,
    color_counts: Vec<usize>,
}

impl Solver {
    fn solve(&mut self) {
        input! {
            N: usize, // node count
            M: usize, // edge count
            G: [(usize, usize); M],
        }

        let mut graph = vec![vec![]; N + 9];
        for (u, v) in G {
            graph[u].push(v);
            graph[v].push(u);
        }

        self.colors = vec![COLOR_NONE; N + 9];

        let mut invalid_count = 0;

        for i in 1..=N {
            if self.colors[i] != COLOR_NONE {
                continue; // この連結成分は既にチェック済み
            }

            // この連結成分内に出現する色のカウント
            self.color_counts = vec![0; 3];

            // どこかの連結成分が二部グラフじゃなかったら、全体が二部グラフじゃないので諦める
            if self.dfs(&graph, i, COLOR_RED) == false {
                println!("0");
                return;
            }

            for count in self.color_counts.iter() {
                // 同一連結成分内の同色のノード同士に辺をつけると二部グラフにならない。
                // そのため、同一連結成分内の同色のノード同士を2個選ぶ組み合わせをカウントする。
                invalid_count += self.combination_count_x_2(*count);
            }
        }

        let all_count = self.combination_count_x_2(N);

        // 存在しうる全ての辺から、既存の辺と、二部グラフにならなくなってしまう辺の数を引く。
        println!("{}", all_count - M - invalid_count);
    }

    fn combination_count_x_2(&self, x: usize) -> usize {
        if x <= 1 {
            return 0;
        }
        x * (x - 1) / 2
    }

    fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, c: usize) -> bool {
        if self.colors[v] != COLOR_NONE {
            return self.colors[v] == c; // 既に着色済みなら、同じ色かどうかチェック。違ったら二部グラフじゃない。
        }

        self.colors[v] = c; // 着色
        self.color_counts[c] += 1; // 連結グラフ内で色をカウント

        for n in graph[v].iter() {
            let next_color = match c {
                COLOR_RED => COLOR_BLUE,
                COLOR_BLUE => COLOR_RED,
                _ => panic!("invalid color"),
            };

            if !self.dfs(graph, *n, next_color) {
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut solver = Solver {
        colors: vec![],
        color_counts: vec![],
    };
    solver.solve();
}
