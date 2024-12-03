use std::io::{self, BufRead};

fn input_nums() -> Vec<i32> {
    let mut line: String = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let nums = input_nums();
    let t = nums[0];
    let mut ans: Vec<i32> = Vec::new();
    for _ in 0..t {
        let nums = input_nums();
        let _n = nums[0];
        let a = input_nums();
        let b = input_nums();
        let c = input_nums();
        let mut a: Vec<(usize, i32)> = a.iter().enumerate().map(|(i, &x)| (i, x)).collect();
        let mut b: Vec<(usize, i32)> = b.iter().enumerate().map(|(i, &x)| (i, x)).collect();
        let mut c: Vec<(usize, i32)> = c.iter().enumerate().map(|(i, &x)| (i, x)).collect();
        a.sort_by(|a, b| b.1.cmp(&a.1));
        b.sort_by(|a, b| b.1.cmp(&a.1));
        c.sort_by(|a, b| b.1.cmp(&a.1));
        // println!("a: {:?}", a);
        // println!("b: {:?}", b);
        // println!("c: {:?}", c);
        let mut res = 0;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    if a[i].0 != b[j].0 && a[i].0 != c[k].0 && b[j].0 != c[k].0 {
                        // println!("{:?} {:?} {:?}", a[i], b[j], c[k]);
                        res = res.max(a[i].1 + b[j].1 + c[k].1);
                    }
                }
            }
        }
        ans.push(res);
        // let mut tmp = vec![(0, a[0]), (0, a[1]), (0, a[2]), (1, b[0]), (1, b[1]), (1, b[2]), (2, c[0]), (2, c[1]), (2, c[2])];
        // let mut set = HashSet::new();
        // let mut set2 = HashSet::new();
        // tmp.sort_by(|a, b| b.1.1.cmp(&a.1.1));
        // // println!("{:?}", tmp);
        // for (j, (i, x)) in tmp {
        //     if set2.insert(i) && set.insert(j)  {
        //         res.push(x);
        //     }
        //     if res.len() >= 3 {
        //         break;
        //     }
        // }
        // ans.push(res.iter().sum());
    
    }
    for x in ans {
        println!("{x}");
    }
}