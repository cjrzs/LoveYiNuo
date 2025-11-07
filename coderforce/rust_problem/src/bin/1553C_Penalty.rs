use std::io::{self, BufRead};

fn input_nums() -> String {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line
}

fn func(nums: &Vec<char>, awin: bool) -> usize {
    let (mut a, mut b) = (5, 5);
    let (mut ascore, mut bscore) = (0, 0);
    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 {
            a -= 1;
            if x == '1' || (x == '?' && awin) {
                ascore += 1;
            }
        } else {
            b -= 1;
            if x == '1' || (x == '?' && !awin) {
                bscore += 1;
            }
        }
        if ascore > bscore + b || bscore > ascore + a {
            return i + 1;
        }
    }
    10
}

fn main() {
    let s = input_nums();
    let t = s.trim().parse::<usize>().unwrap();
    let mut ans = Vec::new();
    for _ in 0..t {
        let s = input_nums();
        let nums = s.trim().chars().collect::<Vec<char>>();
        ans.push(func(&nums, true).min(func(&nums, false)));
    }
    for x in ans {
        println!("{}", x);
    }
}

