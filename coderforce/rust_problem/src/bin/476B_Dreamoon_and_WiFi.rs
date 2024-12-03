use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let a: Vec<char> = line.trim().chars().collect();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let b: Vec<char> = line.trim().chars().collect();
    let plus_a = a.iter().filter(|&c| *c == '+').count();
    let sub_a = a.iter().filter(|&c| *c == '-').count();
    let plus_b = b.iter().filter(|&c| *c == '+').count();
    let sub_b = b.iter().filter(|&c| *c == '-').count();
    // println!("{} {} {} {}", plus_a, sub_a, plus_b, sub_b);
    if plus_a < plus_b || sub_a < sub_b {
        println!("{:.11}", 0.000000000000);
    } else {
        let need_plus = plus_a - plus_b;
        let need_sub = sub_a - sub_b;
        let n = b.len() - plus_b - sub_b;
        let mut res = 0;
        // println!("{n}");
        dfs(n.try_into().unwrap(), need_plus.try_into().unwrap(), need_sub.try_into().unwrap(), &mut res);
        println!("{:.11}", res as f64 / (1 << n) as f64);
    }
}

fn dfs(u: i32, need_plus: i32, need_sub: i32, res: &mut i32) {
    if u == 0 {
        if need_plus == 0 && need_sub == 0 {
            *res += 1
        }
        return 
    }
    dfs(u - 1, need_plus - 1, need_sub, res);
    dfs(u - 1, need_plus, need_sub - 1, res);
}
