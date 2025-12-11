use std::collections::{HashSet};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0] as usize;
    let mut ans= Vec::new();
    for _ in 0..t {
        let _n = input_nums()[0];
        let nums = input_nums();
        let q = input_nums();
        let mut res = Vec::new();
        let mut set = HashSet::new();
        for x in q {
            if !set.contains(&(x - 1)) {
                let mut k = x - 1;
                while !set.contains(&k) {
                    set.insert(k);
                    k = nums[k as usize] - 1;
                }
            }
            res.push(set.len());
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}


