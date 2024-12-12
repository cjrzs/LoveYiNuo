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
    let mut s = vec![0; 200006];
    for i in 0..200005 {
        s[i + 1] = s[i] + find(i as i32 + 1);
    }
    // println!("{:?}", &s[0..8]);
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let [l, r] = [nums[0], nums[1]];
        let res = find(l as i32) + s[r as usize] - s[l as usize - 1];
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}

fn find(x: i32) -> i32 {
    let mut res = 0;
    let mut t = x;
    while t > 0 {
        res += 1;
        t /= 3;
    }
    res
}


