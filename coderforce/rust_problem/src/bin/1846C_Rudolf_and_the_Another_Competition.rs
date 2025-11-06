use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let (n, _m, h) = (nums[0], nums[1], nums[2]);
        let mut tmp = Vec::new();
        for i in 0..n {
            // let mut h2 = h.clone();
            let mut nums = input_nums();
            nums.sort();
            let mut score = 0;
            let mut current_time = 0;
            let mut penalty = 0;

            for &x in &nums {
                if current_time + x <= h {
                    current_time += x;
                    score += 1;
                    penalty += current_time;
                } else {
                    break;
                }
            }
            tmp.push((i, score, penalty));
        }
        tmp.sort_by(|a, b| {
            b.1.cmp(&a.1).then(a.2.cmp(&b.2))
        });
        // println!("{:?}", tmp);
        if let Some(item) = tmp.iter().position(|&(i, _, _)| i == 0) {
            ans.push(item);
        }
    }
    for x in ans {
        println!("{}", x + 1);
    }
}


