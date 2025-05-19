use std::{io::{self, BufRead}, process::exit};

fn input_nums() -> Vec<char> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().chars().collect()
}

fn main() {
    let s = input_nums();
    let n = s.len();
    let mut ab = 100005;
    let mut ba = 100005;
    for i in 1..n {
        if ab == 100005 && s[i - 1] == 'A' && s[i] == 'B' {
            ab = i;
        }
        if ba == 100005 && s[i - 1] == 'B' && s[i] == 'A' {
            ba = i;
        }
    }
    // println!("{} {}", ab, ba);
    if ab != 100005 {
        for i in (ab + 1)..n {
            if s[i] == 'B' && i + 1 < n && s[i + 1] == 'A' {
                println!("YES");
                exit(0);
            }
        }
    }
    if ba != 100005 {
        for i in (ba + 1)..n {
            if s[i] == 'A' && i + 1 < n && s[i + 1] == 'B' {
                println!("YES");
                exit(0);
            }
        }
    }
    println!("NO");
}


