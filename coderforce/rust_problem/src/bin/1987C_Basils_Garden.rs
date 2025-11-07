use std::io::{self, BufRead};
 
fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}
 
fn main() {
    let t = input_nums()[0];
    let mut res = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0];
        let nums = input_nums();
        if n == 1 {
            res.push(nums[0]);
            continue;
        }
        let mut ans = nums[n - 1];
        for i in (0..=n-2).rev() {
            ans = (ans + 1).max(nums[i]);
        }
        res.push(ans);
    }
    for x in res {
        println!("{}", x);
    }
}
 