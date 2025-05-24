use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let t = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (mut l, mut r) = (nums[0], nums[1]);
        let mut res = 0;
        while l != 0 || r != 0 {
            res += r - l;
            r /= 10;
            l /= 10;
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}