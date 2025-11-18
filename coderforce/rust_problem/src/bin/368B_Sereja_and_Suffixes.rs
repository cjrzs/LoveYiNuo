use std::{collections::{HashMap}, io::{BufRead, stdin}};

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let (n, m) = (nums[0], nums[1]);
    let nums = input_nums();
    let mut ans = Vec::new();
    let mut count = HashMap::new();
    let mut t = vec![1; n + 1];
    let mut sum = 0;
    for i in 0..n {
        if !count.contains_key(&nums[i]) {
            sum += 1;
        }
        *count.entry(nums[i]).or_insert(0) += 1;
    }
    for (i, x) in nums.iter().enumerate() {
        if let Some(v) = count.get_mut(x) {
            t[i + 1] = sum;
            if *v > 0 {
                *v -= 1;
            }
            if *v == 0 {
                sum -= 1;
            }
        }
    }
    for _ in 0..m {
        let l = input_nums()[0];
        ans.push(t[l]);
    }
    for x in ans {
        println!("{}", x);
    }
}

