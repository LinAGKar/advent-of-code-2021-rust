use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().map(|height| {
        height.to_digit(10).unwrap()
    }).collect()).collect();

    let mut used: Vec<Vec<bool>> = map.iter().map(|line| vec![false; line.len()]).collect();
    let mut basins = Vec::new();
    let mut queue = Vec::new();

    for (y, line) in map.iter().enumerate() {
        for (x, &height) in line.iter().enumerate() {
            if height == 9 || used[y][x] {
                continue;
            }

            queue.push((y, x));
            let mut size = 0;

            while let Some((y, x)) = queue.pop() {
                if used[y as usize][x as usize] {
                    continue;
                }

                size += 1;
                used[y][x] = true;

                for (dy, dx) in [
                    (-1, 0),
                    (1, 0),
                    (0, -1),
                    (0, 1),
                ] {
                    let y = y as isize + dy;
                    let x = x as isize + dx;

                    if y >= 0 && x >= 0 &&
                       (y as usize) < map.len() && (x as usize) < line.len() &&
                       map[y as usize][x as usize] != 9 {
                        queue.push((y as usize, x as usize));
                    }
                }
            }

            basins.push(size);
        }
    };

    basins.sort_unstable();
    println!("{}", basins.iter().rev().take(3).product::<usize>());
}
