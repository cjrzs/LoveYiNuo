use std::{collections::HashSet, io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0];
        let nums = input_nums();
        if n == 2 {
            ans.push(nums[0].max(nums[1]) + 1);
            // println!("k:{} {}", k, nums[0].max(nums[1]) + 1);
            continue;
        }
        let mut tmp = Vec::new();
        let mut q = 2;
        for _ in 1..=57 {
            tmp.push(q);
            q *= 2;
        }
        // println!("tmp: {:?}", tmp);
        for i in tmp {
            let mut tmp2 = HashSet::new();
            for j in 0..n {
                tmp2.insert(nums[j as usize] % i);
            }
            if tmp2.len() == 2 {
                ans.push(i);
                // println!("k: {} {}", k, i);
                break;
            }
        }
    }
    for x in ans {
        println!("{}", x);
    }
}

