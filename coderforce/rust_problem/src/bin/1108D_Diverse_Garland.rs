use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();
    let n = reader.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut s: Vec<char> = reader.next().unwrap().unwrap().chars().collect();
    let mut cnt = 0;
    for i in 1..n {
        if s[i] == s[i - 1] {
            for x in vec!['R', 'B', 'G'] {
                if i != n - 1 {
                    if x != s[i + 1] && x != s[i] {
                        s[i] = x;
                        break;
                    }
                } else {
                    if x != s[i - 1] {
                        s[i] = x;
                        break;
                    }
                }
            }
            cnt += 1;
        }
    }
    let ans: String = s.into_iter().collect();
    println!("{}", cnt);
    println!("{}", ans);
}

