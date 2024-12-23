use std::io::{self, BufRead};

fn input_nums() -> Vec<i32> {
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
        let [n, x] = [nums[0], nums[1]];
        let mut nums = input_nums();
        let mut res = 0;
        nums.sort();
        let mut cur = 1;
        for i in (0..n).rev() {
            if nums[i as usize] * cur >= x {
                res += 1;
                cur = 1;
            } else {
                cur += 1;
            }
        }
        ans.push(res);
    }
    for x in ans {
        println!("{x}");
    }
}
