use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let s: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let n = s.len();
    let mut d = vec![0; n + 1];
    for i in 1..n {
        d[i] = d[i - 1];
        if s[i] == s[i - 1] {
            d[i] += 1;
        }
    }
    // println!("{:?}", d);
    let t = lines.next().unwrap().unwrap().parse().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums: Vec<usize> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (l, r) = (nums[0], nums[1]);
        ans.push(d[r - 1] - d[l - 1])
    }
    for x in ans {
        println!("{}", x);
    }
}

