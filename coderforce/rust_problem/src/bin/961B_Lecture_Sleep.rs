use std::{collections::VecDeque, io::BufRead};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let (n, k) = (nums[0] as usize, nums[1] as usize);
    let nums = input_nums();
    let arr = input_nums();
    let mut q = VecDeque::new();
    let mut total = 0;
    let mut topk = 0;
    for i in 0..n {
        if arr[i] == 1 {
            total += nums[i];
        } else {
            if i < k {
                topk += nums[i];
            }
        }
        if i < k {
            q.push_back(i);
        }
    }
    let mut res = topk;
    for i in k..n {
        let tmp = q.pop_front().unwrap();
        if arr[tmp] == 0 {
            topk -= nums[tmp];
        }
        if arr[i] == 0 {
            // println!("i {} arr[i] {} {}", i, arr[i], nums[i]);
            topk += nums[i];
        }
        // println!("topk {}", topk);
        res = res.max(topk);
        // println!("res {}", res);
        q.push_back(i);
    }
    println!("{}", total + res);
}
