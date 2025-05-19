use std::io::{self, BufRead};

fn input_nums() -> i64 {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().parse::<i64>().unwrap()
}

fn main() {
    let mut num = input_nums();
    let mut res = Vec::new();
    while num > 0 {
        let mut new_num = Vec::new();
        let mut t = num.clone();
        while t > 0 {
            new_num.push(if t % 10 > 0 {"1"} else {"0"});
            t /= 10;
        }
        let q = new_num
                            .into_iter()
                            .rev()
                            .collect::<Vec<&str>>()
                            .join("")
                            .parse::<i64>()
                            .unwrap();
        res.push(q);
        num -= q;
        // join("")..parse::<i64>().unwrap();
    }
    println!("{}", res.len());
    println!("{}", res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    
}


