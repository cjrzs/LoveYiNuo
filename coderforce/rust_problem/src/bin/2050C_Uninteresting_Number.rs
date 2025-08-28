fn input_nums() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let nums = input_nums();
    let t = nums.split_whitespace().next().unwrap().parse::<i64>().unwrap();
    let mut res = Vec::new();
    for _ in 0..t {
        let s = input_nums();
        let nums: Vec<u32> = s.trim().chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut sum = 0;
        let mut two = 0;
        let mut three = 0;
        for x in nums {
            sum += x;
            if x == 2 {
                two += 1;
            }
            if x == 3 {
                three += 1;
            }
        }
        if sum % 9 == 0 {
            res.push("YES");
            continue;
        }
        // println!("{}", sum);
        // println!("{}", two);
        // println!("{}", three);
        let mut yes = false;
        for i in 0..two.min(8) + 1 {
            for j in 0..three.min(8) + 1 {
                if (sum + 2 * i + 6 * j) % 9 == 0 {
                    // println!("two: {}", i);
                    // println!("three: {}", j);
                    yes = true;
                    break;
                }
            }
        }
        for i in 0..three.min(8) + 1 {
            for j in 0..two.min(8) + 1 {
                if (sum + 2 * j + 6 * i) % 9 == 0 {
                    // println!("three: {}", i);
                    // println!("two: {}", j);
                    yes = true;
                    break;
                }
            }
        }
        if yes {
            res.push("YES");
        } else {
            res.push("NO");
        }
    }
    for x in res {
        println!("{}", x);
    }
}
