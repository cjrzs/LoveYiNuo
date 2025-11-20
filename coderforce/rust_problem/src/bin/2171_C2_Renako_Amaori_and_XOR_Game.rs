use std::{io::{self, BufRead}};

fn input_nums() -> Vec<i64> {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let t = input_nums()[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let n = input_nums()[0];
        let a = input_nums();
        let b = input_nums();
        let mut xor = 0;
        for i in 0..n {
            xor ^= a[i as usize] ^ b[i as usize];
        }
        if xor == 0 {
            ans.push("TIE");
            continue;
        }
        let mut high_bit = 0;
        for i in (0..=20).rev() {
            if (xor >> i) & 1 == 1 {
                high_bit = i;
                break;
            }
        }
        let mut winner = -1;
        for i in (0..n).rev() {
            if ((a[i as usize] ^ b[i as usize]) >> high_bit) & 1 == 1 {
                winner = i + 1;
                break;
            }
        }
        if winner % 2 == 1 {
            ans.push("Ajisai");
        } else {
            ans.push("Mai");
        }
        // for i in (0..=20).rev() {
        //     let mut odd= 0;
        //     let mut even = 0;
        //     for j in 0..n as usize {
        //         let ai = (a[j as usize] >> i) & 1;
        //         let bi = (b[j as usize] >> i) & 1;
        //         if ai != bi {
        //             if (j + 1) % 2 == 1 {
        //                 odd += 1;
        //             } else {
        //                 even += 1;
        //             }
        //         }
        //     }
        //     if odd + even == 0 {
        //         continue;
        //     }
        //     if odd > even {
        //         ans.push("Ajisai");
        //     } else if even > odd {
        //         ans.push("MAI");
        //     } else {
        //         ans.push("Tie");
        //     }
        //     break;
        // }
    }
    for x in ans {
        println!("{}", x);
    }
}

