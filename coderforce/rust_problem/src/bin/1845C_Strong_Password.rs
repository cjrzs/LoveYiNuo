use std::{io::{self, BufRead}};

fn input_nums() -> String {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let t = input_nums()
        .parse()
        .unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let s: Vec<i32> = input_nums()
                            .chars()
                            .map(|x| x.to_string().parse().unwrap())
                            .collect();
        let m = input_nums()
                .parse()
                .unwrap();
        let l: Vec<i32> = input_nums()
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();
        let r: Vec<i32> = input_nums()
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();
        let mut next_pos = 0;
        for i in 0..m {
            let mut max_val_pos = next_pos;
            for d in l[i]..r[i] + 1 {
                let mut val_pos = next_pos;
                while val_pos < s.len() && s[val_pos] != d {
                    val_pos += 1;
                }
                max_val_pos = val_pos.max(max_val_pos)
            }
            next_pos = max_val_pos + 1;
        }
        if next_pos > s.len() {
            ans.push("YES");
        } else {
            ans.push("NO");
        }
    }
    for x in ans {
        println!("{}", x);
    }
}


