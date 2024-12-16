use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans: Vec<i64> = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let n = nums[0];
        let nums = input_nums();
        let mut max_even = 0;
        // let mut max_even_cur = 0;
        let mut max_odd = 0;
        // let mut max_odd_cur = 0;
        let mut flag = 1;
        for i in 0..n {
            flag &= if nums[i as usize] < 0 {1} else {0};
            if i & 1 == 1 {
                max_odd += nums[i as usize].max(0);
            } else {
                max_even += nums[i as usize].max(0);
            }
        }
        if flag == 1 {
            ans.push(*nums.iter().max().unwrap());
        } else {
            ans.push(max_even.max(max_odd));
        }
    }
    for x in ans {
        println!("{x}");
    }
}
