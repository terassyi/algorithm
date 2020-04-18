fn main() {
    println!("Hello, world!");
}

fn dfs(field: &mut Vec<Vec<char>>, n: &usize, m: &usize, x: usize, y: usize) -> Result<(), String> {
    field[x][y] = '.';
    let d: Vec<i32> = vec![-1, 0, 1];
    for dx in d.iter() {
        for dy in d.iter() {
            let nx = if ((x as i32) + dx) >= 0 {
                ((x as i32) + dx) as usize
            } else {
                return Err(String::from("error"))
            };
            let ny = if ((y as i32) + dy) >= 0 {
                ((y as i32) + dy) as usize
            } else {
                return Err(String::from("error"))
            };

            if nx < *n && ny < *m && field[nx][ny] == 'W' {
                match dfs(field, n, m, nx, ny) {
                    Err(e) => println!("{}", e),
                    _ => {},
                }
            }
        }
    }
    Ok(())
}

fn solve(field: &mut Vec<Vec<char>>, n: &usize, m: &usize) -> i32 {
    let mut count: i32 = 0;
    for x in 0..*n {
        for y in 0..*m {
            if field[x][y] == 'W' {
                dfs(field, n, m, x, y);
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: usize = 10;
        let m: usize = 12;
        let mut field: Vec<Vec<char>> = vec![
            vec!['W', '.', '.', '.', '.', '.', '.', '.', '.', 'W', 'W', '.'],
            vec!['.', 'W', 'W', 'W', '.', '.', '.', '.', '.', 'W', 'W', 'W'],
            vec!['.', '.', '.', '.', 'W', 'W', '.', '.', '.', 'W', 'W', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', 'W', 'W', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', 'W', '.', '.'],
            vec!['.', '.', 'W', '.', '.', '.', '.', '.', '.', 'W', '.', '.'],
            vec!['.', 'W', '.', 'W', '.', '.', '.', '.', '.', 'W', 'W', '.'],
            vec!['W', '.', 'W', '.', 'W', '.', '.', '.', '.', '.', 'W', '.'],
            vec!['.', 'W', '.', 'W', '.', '.', '.', '.', '.', '.', 'W', '.'],
            vec!['.', '.', 'W', '.', '.', '.', '.', '.', '.', '.', 'W', '.'],
        ];

        assert_eq!(super::solve(&mut field, &n, &m), 3);
    }
}
