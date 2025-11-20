use std::{io::{self, BufRead}};

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
        let mut nums = input_nums();
        for i in 1..n - 1 {
            if nums[i as usize] == -1 {
                nums[i as usize] = 0;
            }
        }
        if nums[0 as usize] == -1 && nums[n as usize - 1] == -1 {
            nums[0 as usize] = 0;
            nums[n as usize - 1] = 0;
        } else if nums[0 as usize] == -1 {
            nums[0 as usize] = nums[n as usize - 1];
        } else if nums[n as usize - 1] == -1 {
            nums[n as usize - 1] = nums[0 as usize];
        }
        let mut res = 0;
        for i in 1..n {
            res += nums[i as usize] - nums[i as usize - 1];
        }
        ans.push((res.abs(), nums));
    }
    for x in ans {
        println!("{}", x.0);
        println!("{}", x.1.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}
