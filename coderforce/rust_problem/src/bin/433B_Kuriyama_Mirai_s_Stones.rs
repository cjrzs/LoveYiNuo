use::std::io::{self, BufRead};

fn input_nums() -> Vec<usize>{
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    // println!("hello world");
    let n = input_nums()[0];
    let nums = input_nums();
    let mut sorted = nums.to_vec();
    sorted.sort();
    let q: usize = input_nums()[0];
    let mut s1 = vec![0; n + 1];
    for i in 0..n {
        s1[i + 1] = s1[i] + nums[i];
    }
    let mut s2 = vec![0; n + 1];
    for i in 0..n {
        s2[i + 1] = s2[i] + sorted[i];
    }
    let mut arr = Vec::new();
    for _ in 0..q {
        let nums = input_nums();
        if let [k, l, r] = nums.as_slice() {
            arr.push((*k, *l, *r));
        }
    }
    for (k, l, r) in arr {
        // println!("{} {} {}", k, l, r);
        if k == 1 {
            println!("{}", s1[r] - s1[l - 1]);
        } else if k == 2 {
            println!("{}", s2[r] - s2[l - 1]);
        }
    }
}



