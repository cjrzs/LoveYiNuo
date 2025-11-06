use std::{collections::HashMap, io::BufRead};

fn input_nums() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let n = input_nums()[0] as usize;
    let nums = input_nums();
    let mut d = vec![0; 6];
    let mut q = HashMap::new();
    q.insert(4, 0);
    q.insert(8, 1);
    q.insert(15, 2);
    q.insert(16, 3);
    q.insert(23, 4);
    q.insert(42, 5);
    for i in 0..n {
        if nums[i] == 4 {
            d[q[&nums[i]]] += 1;
        } else {
            if d[q[&nums[i]] - 1] > 0 {
                d[q[&nums[i]] - 1] -= 1;
                d[q[&nums[i]]] += 1;
            }
        }
    }
    println!("{}", n - 6 * d[5]);
}

