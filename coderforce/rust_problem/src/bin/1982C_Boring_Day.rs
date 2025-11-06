use std::io::{self, BufRead};

fn input_nums() -> Vec<usize> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let (n, l, r) = (nums[0], nums[1], nums[2]);
        let nums = input_nums();
        let mut cur = 0;
        let mut res = 0;
        let (mut i, mut j) = (0, 0);
        while i < n {
            while j < n && cur < l {
                cur += nums[j];
                j += 1;
            }
            if cur >= l && cur <= r {
                res += 1;
                cur = 0;
                i = j;
            } else {
                cur -= nums[i];
                i += 1;
            }
        }
        ans.push(res);
    }
    for x in ans {
        println!("{}", x);
    }
}


