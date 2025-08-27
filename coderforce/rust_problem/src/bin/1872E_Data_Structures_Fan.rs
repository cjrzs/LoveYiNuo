use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let nums: String = input_nums();
    let t: i32 = nums.parse().unwrap();
    let mut ans: Vec<String> = Vec::new();
    for _ in 0..t {
        let nums: String = input_nums();
        let n: usize = nums.parse().unwrap();
        let nums = input_nums();
        let arr: Vec<i32> = nums.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let s: Vec<i32> = input_nums().chars().map(|x| x.to_string().parse().unwrap()).collect();
        let mut x1 = 0;
        let mut x0 = 0;
        let mut res = Vec::new();
        let mut p = vec![0; n + 1];
        for (i, x) in s.iter().enumerate() {
            if *x == 0 {
                x0 ^= arr[i];
            } else {
                x1 ^= arr[i];
            }
            p[i + 1] = p[i]  ^ arr[i];
        }
        // println!("p: {:?}", p);
        let qs = input_nums();
        for _ in 0..qs.parse().unwrap() {
            let input = input_nums();
            let q: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
            if q[0] == 2 {
                res.push(if q[1] == 0 {x0.to_string()} else {x1.to_string()});
            } else {
                let (l, r) = (q[1], q[2]);
                x0 ^= p[r as usize] ^ p[l as usize - 1];
                x1 ^= p[r as usize] ^ p[l as usize - 1];
            }
        }
        ans.push(res.join(" "));
    }
    for x in ans {
        println!("{}", x);
    }
}




