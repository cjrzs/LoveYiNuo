fn input_nums() -> Vec<usize> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn dfs(u: usize, v: usize, edges: &Vec<Vec<(usize, usize)>>, color: usize, vis: &mut Vec<bool>) -> bool{
    if u == v {
        return true;
    }
    for &(next, c) in &edges[u] {
        if vis[next] {
            continue;
        }
        if c == color {
            vis[next] = true;
            if dfs(next, v, edges, color, vis) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let nums = input_nums();
    let (n, m) = (nums[0], nums[1]);
    let mut edges: Vec<Vec<(usize, usize)>> = vec![vec![]; n + 1];
    
    for _ in 0..m {
        let nums = input_nums();
        let u = nums[0];
        let v = nums[1];
        let color = nums[2];
        edges[u].push((v, color));
        edges[v].push((u, color));
    }
    let q = input_nums()[0];
    let mut res = Vec::new();
    for _ in 0..q {
        let nums = input_nums();
        let (u, v) = (nums[0], nums[1]);
        let mut ans = 0;
        for i in 1..=m {
            let mut vis = vec![false; 105];
            if !vis[u] {
                vis[u] = true;
                if dfs(u, v, &edges, i, &mut vis) {
                    // println!("{}" , i);
                    ans += 1;
                }
            }
        }
        res.push(ans);
    }
    for x in res {
        println!("{}", x);
    }
}

