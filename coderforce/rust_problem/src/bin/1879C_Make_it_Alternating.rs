use std::io::{self, BufRead};

const MOD: i64 = 998244353;

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
    let mut fac: Vec<i64> = vec![0; 200001];
    fac[0] = 1;
    for i in 1..200001 {
        fac[i] = fac[i - 1] * i as i64 % MOD;
    }
    for _ in 0..t {
        let nums: Vec<char> = input_nums().chars().collect();
        let mut res1 = 0;
        let mut res2 = 1;
        let mut u: i64 = 1;
        // let mut tmp: i64 = 0;
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                res1 += 1;
            }
        }
        if res1 == 0 {
            ans.push((res1, 1));
            continue;
        }
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                u += 1;
            } else {
                if u > 1 {
                    // println!("u: {}", u);
                    res2 = res2 * u % MOD;
                }
                u = 1;
            }
        }
        if u > 1 {
            res2 = res2 * u % MOD;
        }
        ans.push((res1, res2 * fac[res1 as usize] % MOD));
    }
    for (x, y) in ans {
        println!("{} {}", x, y);  
    }
}

