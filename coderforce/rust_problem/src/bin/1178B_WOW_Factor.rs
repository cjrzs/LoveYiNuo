use std::io::BufRead;

fn input_nums() -> Vec<char> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().chars().collect()
}

fn main() {
    let nums = input_nums();
    let n = nums.len();
    let (mut cur, mut v , mut res) = (0i64, 0, 0);
    for i in 1..n {
        if nums[i] == 'o' {
            cur += v;
        } else if nums[i - 1] == 'v' {
            v += 1;
            res += cur;
        }
    }
    println!("{}", res);
}

