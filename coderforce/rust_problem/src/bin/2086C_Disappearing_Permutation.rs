use std::collections::{HashMap, HashSet};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn dfs(nums: &Vec<i64>, u: usize, hash: &HashMap<i64, i64>) {
    if 
}

fn main() {
    let t = input_nums()[0] as usize;
    for _ in 0..t {
        let n = input_nums()[0];
        let nums = input_nums();
        let q = input_nums();
        let mut res = Vec::new();
        let mut hash = HashMap::new();
        for x in q {
            let i = x - 1;
            if hash.contains_key(&i) {
                res.push(hash[&i]);
            } else {
                hash
            }
        }
    }
}


