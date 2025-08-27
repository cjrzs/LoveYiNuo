fn input_nums() -> Vec<i32> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = input_nums()[0] as usize;
    let size_nums = input_nums();
    let val_nums = input_nums();
    let mut ans = i32::MAX;
    for j in 0..n {
        let mut min_left = i32::MAX;
        let mut min_right = i32::MAX;
        for i in 0..j {
            if size_nums[i] < size_nums[j] {
                min_left = min_left.min(val_nums[i]);
            }
        }
        for k in j + 1..n {
            if size_nums[j] < size_nums[k] {
                min_right = min_right.min(val_nums[k]);
            }
        }
        if min_left == i32::MAX || min_right == i32::MAX {
            continue;
        }
        // println!("{} {} {}", min_left, min_right, val_nums[j]);
        ans = ans.min(min_left + min_right + val_nums[j]);
    }
    println!("{}", if ans == i32::MAX { -1 } else { ans });
}

