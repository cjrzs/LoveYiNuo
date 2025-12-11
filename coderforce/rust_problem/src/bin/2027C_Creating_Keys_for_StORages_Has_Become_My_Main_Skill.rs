use std::io::{self, BufRead};

fn input_nums() -> Vec<i64> {
    let mut line: String = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let (n, x) = (nums[0], nums[1]);
        let mut cur = 0;
        let mut res = vec![x; n as usize];
        let mut flag = true;
        for i in 0..=n - 2 {
            if ((cur | i) & x) == (cur | i) {
                cur |= i;
                res[i as usize] = i;
            } else {
                flag = false;
                break;
            }
        }
        if flag && (cur | (n - 1) == x) {
            res[n as usize - 1] = n - 1;
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}

