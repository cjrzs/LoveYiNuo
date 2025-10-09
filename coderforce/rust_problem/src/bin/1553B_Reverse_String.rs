use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut ans = Vec::new();
    let n = line.trim().parse().unwrap();
    for _ in 0..n {
        let s: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
        let t: Vec<char> = lines.next().unwrap().unwrap().trim().chars().collect();
        let n = s.len();
        let m = t.len();
        let mut flag = false;
        // 从 位置i 开始走
        for i in 0..n {
            // 走 j 步
            for j in 0..n - i {
                if j >= m {
                    break;
                }
                let k = m - 1 - j; // 还剩下 k 步
                // i+j 是当前最右下标的位置。往走左k步，如果不够k就不能走了。
                if i + j < k {
                    continue;
                }
                let mut path = Vec::new();
                path.extend_from_slice(&s[i..=i + j]);
                if k > 0 {
                    path.extend(s[i + j - k..i + j].iter().rev());
                }
                // println!("{:?} {:?}", path, t);
                if path == t {
                    flag = true;
                    break;
                }
            }
            if flag {
                break;
            }
        }
        ans.push(if flag {"YES"} else {"NO"});
    }
    for x in ans {
        println!("{}", x);
    }
}

