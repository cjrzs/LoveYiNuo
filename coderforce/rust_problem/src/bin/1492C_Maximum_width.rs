use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nums = input_nums();
    let q: Vec<usize> = nums.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (n, m) = (q[0], q[1]);
    let nums = input_nums();
    let s: Vec<char> = nums.chars().collect();
    let nums = input_nums();
    let t: Vec<char> = nums.chars().collect();
    let mut left = vec![100005; m];
    let mut right = vec![100005; m];
    let (mut i, mut j) = (0, 0);
    while i < n {
        if j < m && t[j] == s[i] {
            left[j] = i;
            j += 1;
        }
        i += 1;
    }
    let (mut i, mut j) = (n, m);
    while i > 0 {
        i -= 1;
        if j > 0 && t[j - 1] == s[i] {
            j -= 1;
            right[j] = i;
        }
    }
    // println!("{:?} \n {:?}", left, right);
    let mut res = 0;
    for i in 0..m - 1 {
        res = res.max(right[i + 1] - left[i]);
    }
    println!("{}", res);
}


