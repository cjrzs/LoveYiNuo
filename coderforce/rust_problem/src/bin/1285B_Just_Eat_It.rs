use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn func(nums: &[i64], n: usize) -> i64 {
    let mut maxx: i64 = 0;
    let mut so_far: i64 = 0;
    for i in 0..n {
        maxx = nums[i].max(nums[i] + maxx);
        so_far = so_far.max(maxx);
    }
    so_far
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n = nums[0];
        let nums: Vec<i64> = input_nums();
        let s: i64 = nums.iter().sum();
        let max1 = func(&nums[1..n as usize], n as usize - 1);
        let max2 = func(&nums[0..n as usize - 1], n as usize - 1);
        // println!("{} {} {}", max1, max2, s);
        let q = max1.max(max2);
        if q < s {
            ans.push("YES")
        } else {
            ans.push("NO");
        }   
    }
    for x in ans {
        println!("{}", x);
    }
}