use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nums = input_nums();
    let t = nums.parse().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let nums: Vec<i32> = nums.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (a, b) = (nums[0], nums[1]);
        let nums = input_nums();
        let nums: Vec<char> = nums.chars().collect();
        let mut bucket = Vec::new();
        let mut i: usize = 0;
        let mut res = 0;
        let n: usize = nums.len();
        let mut flag = true;
        while i < n {
            let t = i;
            while i < n && nums[i] == '1' {
                i += 1;
                if flag {
                    flag = false;
                }
            }
            if i > t {
                bucket.push((t, i));
            }
            i += 1;
        }
        // println!("{:?}", bucket);
        for i in 1..bucket.len() {
            let combind = (bucket[i].0 - bucket[i - 1].1) * b as usize;
            if combind < a as usize {
                res += combind;
            } else {
                res += a as usize;
            }
        }
        ans.push(if flag {0} else {res + a as usize});
    }
    for x in ans {
        println!("{x}");
    }
}


