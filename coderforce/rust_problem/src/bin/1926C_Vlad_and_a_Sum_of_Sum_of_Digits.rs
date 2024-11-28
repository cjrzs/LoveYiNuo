use std::io::{self, BufRead};

fn sum(x: i32) -> i32 {
    let mut t = x;
    let mut res: i32 = 0;
    while t > 0 {
        res += t % 10;
        t /= 10;
    }
    res
}

fn build() -> Vec<i32> {
    let mut ans = Vec::new();
    for x in 1..200005 {
        ans.push(sum(x));
    }
    let mut res: Vec<i32> = vec![0; 200006];
    for (i, x) in ans.iter().enumerate() {
        if i > 0 {
            res[i] = res[i - 1] + x;
        }
    }
    res
}

fn main() {
    let ans = build();
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    // println!("line: {}", line);
    let t = line.trim().parse().unwrap();
    // println!("t: {}", t);
    let mut res: Vec<i32> = Vec::new();
    for _ in 0..t {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let x: i32 = line.trim().parse::<i32>().unwrap();
        // println!("val of x: {}", x);
        res.push(ans[x as usize - 1] + 1);
    }
    for x in res {
        println!("{}", x);
    }
}