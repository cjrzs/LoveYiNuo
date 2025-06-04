use std::io;
use std::cmp::max;

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let n = input_nums()[0];
    let a = input_nums();
    let mut start = vec![1; n];
    let mut end = Vec::new();
    end.push(1);
    let mut cnt = 1;
    for i in 1..n {
        if a[i] > a[i - 1] {
            cnt += 1;
        } else {
            cnt = 1;
        }
        end.push(cnt);
    }
    for i in (0..n - 1).rev() {
        if a[i] < a[i + 1] {
            start[i] = start[i + 1] + 1;
        }
    }
    let mut res = 0;
    for i in 1..n - 1 {
        if a[i - 1] < a[i + 1] {
            // println!("{} {} {}", i, end[i - 1], start[i + 1]);
            res = max(res, end[i - 1] + start[i + 1]);
        }
    }
    if n == 2 {
        println!("{}", if a[0] < a[1] { 2 } else { 1 });
    } else {
        for i in 0..n {
            res = max(res, end[i]);
        }
        println!("{}", res);
    }
}
