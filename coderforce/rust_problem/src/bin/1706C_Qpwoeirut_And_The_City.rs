use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n = nums[0];
        let nums = input_nums();
        let mut tmp = Vec::new();
        for i in (1..n - 1).step_by(2) {
            let k = nums[i as usize - 1 ].max(nums[i as usize + 1]);
            if nums[i as usize] <= k {
                tmp.push(k + 1 - nums[i as usize]);
            } else {
                tmp.push(0);
            }
        }
        // println!("tmp: {:?}", tmp);
        let mut s1 = vec![0; tmp.len() + 1];
        for (i, x) in tmp.iter().enumerate() {
            s1[i + 1] = s1[i] + x;
        }
        // println!("s1: {:?}", s1);
        let mut one: i64 = tmp.iter().sum();
        if n % 2 == 0 {
            let mut tmp2 = Vec::new();
            for i in (2..n - 1).step_by(2) {
                let k = nums[i as usize - 1].max(nums[i as usize + 1]);
                if nums[i as usize] <= k {
                    tmp2.push(k + 1 - nums[i as usize]);
                } else {
                    tmp2.push(0);
                }
            }
            let mut s2: Vec<i64> = vec![0; tmp.len() + 1];
            for i in (0..tmp2.len()).rev() {
                s2[i] = s2[i + 1] + tmp2[i];
            }
            // println!("tmp2: {:?}", tmp2);
            // println!("s2: {:?}", s2);
            let mut zero = i64::MAX;
            for i in 0..tmp.len() {
                zero = zero.min(s1[i as usize] + s2[i]);
            }
            one = one.min(zero);
        }
        ans.push(one);
    }
    for x in ans {
        println!("{}", x);
    }
}


