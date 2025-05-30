use std::io::{self, BufRead};

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect()
}

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n + 1],
        }
    }
    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }   
}

fn dfs(g: &Graph, u: usize, p: usize) -> f64 {
    let mut cnt = 0;
    let mut res: f64 = 0.0;
    for v in g.adj[u].iter() {
        if *v != p {
            cnt += 1;
        }
    }
    if cnt == 0 {
        return res;
    }
    for v in g.adj[u].iter() {
        if *v != p {
            res += 1 as f64 / cnt as f64 * (1 as f64 + dfs(g, *v, u));
        }
    }
    return res;
}
fn main() {
    let n = input_nums()[0];
    let mut g = Graph::new(n + 5);
    for _ in 0..n - 1 {
        let nums = input_nums();
        let [u, v] = [nums[0], nums[1]];
        g.add_edge(u, v);
    }
    // println!("{} {}", total, d);
    println!("{:.15}", dfs(&g, 1, 0));
}


