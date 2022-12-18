use proconio::{input, marker::Chars};

static COLOR_NONE: usize = 0;
static COLOR_RED: usize = 1;
static COLOR_BLUE: usize = 2;

fn main() {
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

    let mut colors = vec![COLOR_NONE; N + 9];
    let mut color_counts = vec![0; 3];

    let mut invalid_count = 0;

    for i in 1..=N {
        if colors[i] != COLOR_NONE {
            continue;
        }

        color_counts = vec![0; 3];

        if dfs(&graph, &mut colors, &mut color_counts, i, COLOR_RED) == false {
            println!("0");
            return;
        }

        for count in color_counts {
            invalid_count += combination_count_x_2(count);
        }
    }

    let all_count = combination_count_x_2(N);

    println!("{}", all_count - M - invalid_count);
}

fn combination_count_x_2(x: usize) -> usize {
    if x <= 1 {
        return 0;
    }
    x * (x - 1) / 2
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    colors: &mut Vec<usize>,
    color_counts: &mut Vec<usize>,
    v: usize,
    c: usize,
) -> bool {
    if colors[v] != COLOR_NONE {
        return colors[v] == c; // 既に着色済みなら、同じ色かどうかチェック。違ったら二部グラフじゃない。
    }

    colors[v] = c; // 着色
    color_counts[c] += 1; // 連結グラフ内で色をカウント

    for n in graph[v].iter() {
        let next_color = if c == COLOR_RED {
            COLOR_BLUE
        } else {
            COLOR_RED
        };

        if !dfs(graph, colors, color_counts, *n, next_color) {
            return false;
        }
    }

    true
}
