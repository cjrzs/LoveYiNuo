use std::io::BufRead;

fn input() -> String {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
    line
}

fn main() {
    let t = input().trim().parse::<i32>().unwrap();
    let mut res = Vec::new();
    for _ in 0..t {
        let nums: Vec<char> = input().trim().chars().collect();
        let mut cnt = 0;
        let mut ans = Vec::new();
        let n = nums.len();
        let mut i = 0;
        while i < n {
            if nums[i] == 'o' {
                if i + 1 < n && nums[i + 1] == 'n' && i + 2 < n && nums[i + 2] == 'e' {
                    cnt += 1;
                    ans.push(i + 1);
                    i += 2
                }
            } else if nums[i] == 't' {
                if i + 1 < n && nums[i + 1] == 'w' && i + 2 < n && nums[i + 2] == 'o' {
                    cnt += 1;
                    if i + 3 < n && nums[i + 3] == 'n' && i + 4 < n && nums[i + 4] == 'e' {
                        ans.push(i + 2);
                        i += 4;
                    } else {
                        ans.push(i + 1);
                        i += 2;
                    }
                }
            }
            i += 1;
        }
        res.push((cnt, ans));
    }
    for (cnt, ans) in res {
        println!("{}", cnt);
        for i in ans {
            print!("{} ", i + 1);
        }
        println!();
    }
}


