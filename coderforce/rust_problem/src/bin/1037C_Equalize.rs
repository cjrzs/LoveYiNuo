use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nums = input_nums();
    let n = nums.parse::<usize>().unwrap();
    let a: Vec<char> = input_nums().chars().collect();
    let b: Vec<char> = input_nums().chars().collect();
    let mut res: usize = 0;
    // let mut one_pre: Vec<usize> = Vec::new();
    // let mut zero_pre: Vec<usize> = Vec::new();
    // for i in 0..n {
    //     if a[i] != b[i] {
    //         if a[i] == '1' {
    //             if let Some(idx) = zero_pre.pop() {
    //                 let t = (idx as isize - i as isize).abs() as usize;
    //                 if t >= 2 {
    //                     res += 2;
    //                 } else if t == 1 {
    //                     res += 1;
    //                 }
    //             } else {
    //                 one_pre.push(i);
    //             };
    //         } else {
    //             if let Some(idx) = one_pre.pop() {
    //                 let t = (idx as isize - i as isize).abs() as usize;
    //                 res += if t >= 2 {2} else {1}
    //             } else {
    //                 zero_pre.push(i);
    //             };
    //         }
    //     }
    // }
    // if !one_pre.is_empty() {
    //     res += one_pre.len();
    // }
    // if !zero_pre.is_empty() {
    //     res += zero_pre.len();
    // }
    let mut i = 0;
    while i < n {
        if i + 1 < n && a[i] != b[i] && a[i + 1] != b[i + 1] && a[i] != a[i + 1] {
            res += 1;
            i += 1;
        } else {
            res += if a[i] != b[i] {1} else {0};
        }
        i += 1;
    }
    println!("{}", res);
}

