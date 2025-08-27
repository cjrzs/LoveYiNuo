use std::io::{self, BufRead};

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let (n, s) = (nums[0], nums[1]);
        let nums = input_nums();
        if n == 1 {
            ans.push(s.max(nums[0]) - s.min(nums[0]));
            continue;
        }
        let min_x = nums[0];
        let max_x = nums[n - 1];
        if s > max_x {
            ans.push(s - min_x);
            continue;
        }
        if s < min_x {
            ans.push(max_x - s);
            continue;
        }
        let a = s - min_x;
        let b = max_x - s;
        ans.push(a.min(b) * 2 + a.max(b));
    }
    for x in ans {
        println!("{}", x);
    }
}

