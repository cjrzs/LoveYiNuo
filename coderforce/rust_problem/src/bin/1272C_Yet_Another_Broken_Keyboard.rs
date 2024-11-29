use std::{collections::HashSet, io::{self, BufRead}};

fn input_nums() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

fn main() {
    let nums = input_nums();
    let nums: Vec<i32> = nums.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let _n = nums[0];
    let _m = nums[1];
    let origin = input_nums();
    let used = input_nums();
    let used: Vec<char> = used.trim()
                            .split_whitespace()
                            .map(|s| s.chars().next().unwrap())
                            .collect();
    let set: HashSet<_> = HashSet::from_iter(used);
    let mut res:i64 = 0;
    let mut t = 0;
    for x in origin.chars() {
        if set.contains(&x) {
            t += 1;
        } else {
            res += t * (t + 1) / 2;
            t = 0;
        }
        // println!("t: {t}, res: {res}");
    }
    res += t * (t + 1) / 2;
    println!("{}", res);
}

