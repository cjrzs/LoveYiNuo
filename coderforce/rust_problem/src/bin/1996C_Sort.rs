use std::io::BufRead;

fn main() {
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<usize> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let t = nums[0];
    let mut ans = Vec::new();
    for _ in 0..t {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let nums: Vec<usize> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = nums[0];
        let m = nums[1];
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let a = line.trim();
        let a: Vec<char> = a.chars().collect();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let b = line.trim();
        let b: Vec<char> = b.chars().collect();
        let mut cnta = vec![vec![0; n + 1]; 27];
        let mut cntb = vec![vec![0; n + 1]; 27];
        for i in 0..n {
            let ac = a[i];
            let bc = b[i];
            for c in 'a'..='z' {
                let idx = c as u8 - b'a';
                // if i > 0 {
                    if ac == c {
                        cnta[idx as usize][i + 1] = cnta[idx as usize][i] + 1;
                    } else {
                        cnta[idx as usize][i + 1] = cnta[idx as usize][i];
                    }
                    if bc == c {
                        cntb[idx as usize][i + 1] = cntb[idx as usize][i] + 1;
                    } else {
                        cntb[idx as usize][i + 1] = cntb[idx as usize][i];
                    }
                // }
            }
        }
        // println!("aaa: {:?}, bbb: {:?}", cnta, cntb);
        for _ in 0..m {
            let mut res = 0;
            let mut line = String::new();
            stdin.lock().read_line(&mut line).unwrap();
            let nums: Vec<usize> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            let l = nums[0];
            let r = nums[1];
            for i in 0..26 {
                let qa = cnta[i][r] - cnta[i][l - 1];
                let qb = cntb[i][r] - cntb[i][l - 1];
                if qa > qb {
                    let u: i32 = qa - qb;
                    res += u.abs();
                }
            }
            ans.push(res);
        }
    }
    for x in ans {
        println!("{x}");
    }
    
}
