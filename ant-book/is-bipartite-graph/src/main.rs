fn main() {
    println!("Hello, world!");
}

fn solve(n: usize, neighbor_list: Vec<Vec<usize>>) -> bool {
    let mut colors = vec![0u8; n];
    let mut color: bool = true;
    for (i, vertex) in neighbor_list.iter().enumerate() {
        if !paint(&mut colors, i, color) { return false; }
        color = !color;
        for next in vertex.iter() {
            if !paint(&mut colors, *next, color) { return false; }
        }
    }
    true
}

fn paint(colors: &mut Vec<u8>, index: usize, color: bool) -> bool {
    if colors[index] != 0 {
        if is_color(colors, index) == color { return true; }
        else { return false; }
    }
    if color {
        colors[index] = 1;
    } else {
        colors[index] = 2;
    }
    true
}

fn is_color(colors: &Vec<u8>, index: usize) -> bool {
    if colors[index] == 1 { return true; }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        let n: usize = 3;
        let neighbor_list: Vec<Vec<usize>> = vec![
            vec![1,2],
            vec![0,2],
            vec![0,1],
        ];

        assert_eq!(super::solve(n, neighbor_list), false);
    }

    #[test]
    fn test_solve2() {
        let n: usize = 4;
        let neighbor_list: Vec<Vec<usize>> = vec![
            vec![1,3],
            vec![0,2],
            vec![1,3],
            vec![0,2],
        ];
        assert_eq!(super::solve(n, neighbor_list), true);
    }
}
