use std::{collections::HashSet, io::{self, BufRead}};

fn input_nums() -> String {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main () {
    let nums = input_nums();
    let t = nums.parse().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n: usize = nums.parse().unwrap();
        let mut hash_set = HashSet::new();
        let nums: Vec<char> = input_nums()
            .chars()
            .collect();
        let mut flag = false;
        hash_set.insert(&nums[n - 1]);
        hash_set.insert(&nums[0]);
        for i in 1..n - 1 {
            if hash_set.contains(&nums[i]) {
                flag = true;
            }
            hash_set.insert(&nums[i]);
        }
        if flag {
            ans.push("YES");
        } else {
            ans.push("NO");
        }
    }
    for x in ans {
        println!("{}", x);
    }
}

