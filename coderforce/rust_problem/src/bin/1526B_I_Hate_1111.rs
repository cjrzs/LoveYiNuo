use std::io::{self, BufRead};

fn input_nums() -> i32 {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn main() {
    let t = input_nums();
    let mut ans = Vec::new();
    for _ in 0..t {
        let num = input_nums();
        let t = num % 11 * 111 <= num;
        ans.push(if t {"YES"} else {"NO"});
    }
    for x in ans {
        println!("{x}");
    }
}


