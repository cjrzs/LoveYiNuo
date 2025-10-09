use std::io::BufRead;

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0] as usize;
        let nums = input_nums();
        let mut res = Vec::new();
        if n % 2 != 0 {
            res.push((usize::MAX, usize::MAX));
            ans.push(res);
            continue;
        }
        for i in (1..n).step_by(2) {
            if nums[i - 1] == nums[i] {
                res.push((i, i + 1));
            } else {
                res.push((i, i));
                res.push((i + 1, i + 1));
            }
        }
        ans.push(res);
    }
    for res in ans {
        if res.len() == 1 && res[0].0 == usize::MAX && res[0].1 == usize::MAX {
            println!("{}", -1);
        } else {
            println!("{}", res.len());
            for (x, y) in res {
                println!("{} {}", x, y);
            }
        }
    }
}


