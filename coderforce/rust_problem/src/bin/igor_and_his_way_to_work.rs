use std::collections::VecDeque;
use std::io::{self, BufRead};

fn input_nums() -> Vec<i32> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn bfs(
    startx: usize,
    starty: usize,
    nums: &Vec<Vec<char>>,
    n: usize,
    m: usize,
) -> bool {
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, -1, 0, 1];
    let mut vis = vec![vec![63; m]; n];
    let mut q = VecDeque::new();

    // 初始化起点
    vis[startx][starty] = 0;
    q.push_back((startx, starty, -1i32, 0));

    while let Some((x, y, forward, cnt)) = q.pop_front() {
        // println!("{} {} {} {}", x, y, forward, cnt);
        if nums[x][y] == 'T' {
            return true;
        }

        for i in 0..4 {
            let x_i32 = x as i32;
            let y_i32 = y as i32;
            let new_x = x_i32 + dx[i];
            let new_y = y_i32 + dy[i];
            let mut t = cnt;
            if forward != -1 && forward != i as i32 {
                t += 1;
            }

            if t > 2 {
                continue;
            }


            if new_x < 0 || new_x >= n as i32 || new_y < 0 || new_y >= m as i32 {
                continue;
            }

            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if nums[new_x][new_y] == '*' || vis[new_x][new_y] < t {
                continue;
            }

            vis[new_x][new_y] = t;
            q.push_back((new_x, new_y, i as i32, t));
        }
    }
    false
}

fn main() {
    let nums = input_nums();
    let n = nums[0] as usize;
    let m = nums[1] as usize;

    let stdin = io::stdin();
    let mut nums_grid = vec![vec![' '; m]; n];

    for i in 0..n {
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        nums_grid[i] = line.trim().chars().collect();
    }

    let mut res = false;
    'outer: for i in 0..n {
        for j in 0..m {
            if nums_grid[i][j] == 'S' {
                res = bfs(i, j, &nums_grid, n, m);
                break 'outer;
            }
        }
    }

    println!("{}", if res { "YES" } else { "NO" });
}