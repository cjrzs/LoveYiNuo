use std::{collections::HashMap, io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let n = nums[0];
    let nums = input_nums();
    let m = 4 * 10000 + 5;
    let mut res = 0;
    let mut f: HashMap<i64, Vec<i64>> = HashMap::new();
    for i in 0..n {
        let t = i - nums[i as usize] + m;
        f.entry(t) // 使用 entry 方法
            .or_insert_with(Vec::new) // 如果键不存在，则插入一个新的 Vec<i64>
            .push(nums[i as usize]); // 向 Vec 中添加元素
    }
    for x in f.keys() {
        res = res.max(f.get(x).unwrap().iter().sum());
    }
    println!("{}", res);
}



