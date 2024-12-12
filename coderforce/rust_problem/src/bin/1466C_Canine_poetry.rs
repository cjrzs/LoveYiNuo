use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nums = input_nums();
    let t = nums.parse::<usize>().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let nums: Vec<char> = nums.chars().collect();
        let mut res: usize = 0;
        let n = nums.len();
        let mut i = 0;
        let mut set = std::collections::HashSet::new();
        while i < n {
            if set.contains(&i) {
                i += 1;
                continue;
            }
            if i + 1 < n && nums[i] == nums[i + 1] {
                if i + 2 < n && nums[i] == nums[i + 2] {
                    res += 2;
                    i += 2;
                } else {
                    res += 1;
                    i += 1;
                }
            } else if i + 2 < n && nums[i] == nums[i + 2] {
                res += if nums[i] == nums[i + 1] { 2 } else { 1 };
                set.insert(i + 2);
            }
            i += 1
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}

