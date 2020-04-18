use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

fn bfs(field: &mut Vec<Vec<char>>, 
    size: &(usize, usize),
    start: &(usize, usize), 
    goal: &(usize, usize)) -> usize {
    let dx: Vec<i32> = vec![1, 0, -1, 0];
    let dy: Vec<i32> = vec![0, 1, 0, -1];
    let inf: usize = 100000;

    let mut path: Vec<Vec<(usize)>> = Vec::new();
    
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    // init path
    for i in 0..size.0 {
        let mut tmp: Vec<usize> = Vec::new();
        for j in 0..size.1 {
            tmp.push(inf);
        }
        path.push(tmp);
    }
    path[start.0][start.1] = 0;
    queue.push_back(*start);

    loop {
        let p: (usize, usize) = queue.pop_front().unwrap();
        if p == *goal {
            break;
        }

        // move
        for i in 0..4 {
            let nx = p.0 as i32 + dx[i];
            let ny = p.1 as i32 + dy[i];

            if nx < 0 || ny < 0 {
                continue;
            }
            
            let nx = nx as usize;
            let ny = ny as usize;

            if nx < size.0 && ny < size.1 && field[nx][ny] != '#' && path[nx][ny] == inf {
                queue.push_back((nx, ny));
                path[nx][ny] = path[p.0][p.1] + 1;
            }
        }
    }
    print_path(&path);
    path[goal.0][goal.1]
}

fn print_path(path: &Vec<Vec<usize>>) {
    for v in path.iter() {
        println!("{:?}", v);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_maze() {
        let mut maze: Vec<Vec<char>> = vec![
            vec!['#', 'S', '#', '#', '#', '#', '#', '#', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['.', '#', '.', '#', '#', '.', '#', '#', '.', '#'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '#', '.', '#', '#', '.', '#', '#', '#', '#'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '#', '#', '#', '#', '#', '#', '.', '#'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', '#', '#', '#', '.', '#', '#', '#', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', 'G', '#'],
        ];
        let size: (usize, usize) = (10, 10);
        let start: (usize, usize) = (0, 1);
        let goal: (usize, usize) = (9, 8);

        assert_eq!(super::bfs(&mut maze, &size, &start, &goal), 22);
    }
}
