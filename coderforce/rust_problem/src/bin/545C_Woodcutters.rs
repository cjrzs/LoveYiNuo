use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("不是有效数字");
    let mut data: Vec<(i32, i32)> = Vec::with_capacity(n);
    let mut sum = 2;
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            if parts.len() == 2 {
                data.push((parts[0], parts[1]));
            }
        }
    }
    let mut pre = data[0].0;
    for i in 1..n - 1 {
        let (x, h) = data[i];
        let y = data[i + 1].0;
        // println!("{} {} {} {}", pre, y, x - h, x + h);
        if x - h > pre {
            sum += 1;
            pre = x;
        } else if x + h < y {
            sum += 1;
            pre = x + h;
        } else {
            pre = x;
        }
    }
    println!("{}", if n > 1 {sum} else {1});
}


