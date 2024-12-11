use std::{collections, io::{self, BufRead}};

fn input_nums() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nums = input_nums();
    let t: i32 = nums.parse().unwrap();
    let mut ans: Vec<i32> = Vec::new();
    for _ in 0..t {
        let s = input_nums();
        let mut set = collections::HashSet::new();
        let mut res: i32 = 0;
        for x in s.chars().into_iter() {
            // println!("{}", x);
            // println!("{:?}", set);
            if !set.insert(x) {
                res += 2;
                set.clear();
            }
        }
        // println!("{} {}", res, s.len());
        ans.push(s.len() as i32 - res);
    }
    for x in ans {
        println!("{}", x);
    }
}


