use std::{io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let (n, m, k) = (nums[0], nums[1], nums[2]);
    let mut army = Vec::new();
    for _ in 0..m + 1 {
        army.push(input_nums()[0]);
    }
    let mut res = 0;
    let x = army[m as usize];
    for i in 0..m {
        let mut sum = 0;
        for j in 0..=n {
            if ((x >> j) & 1) ^ ((army[i as usize] >> j) & 1) == 1 {
                sum += 1;
            }
        }
        // println!("numsi {}", army[i as usize]);
        // println!("sum {}", sum);
        if sum <= k {
            res += 1;
        }
    }
    println!("{}", res);
}
