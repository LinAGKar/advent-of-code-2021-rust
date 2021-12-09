use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().map(|height| {
        height.to_digit(10).unwrap()
    }).collect()).collect();

    println!("{}", map.iter().enumerate().flat_map(|(y, line)| {
        let map = &map;

        line.iter().enumerate().filter_map(move |(x, &height)| {
            if (y == 0 || height < map[y - 1][x]) &&
               (y == map.len() - 1 || height < map[y + 1][x]) &&
               (x == 0 || height < map[y][x - 1]) &&
               (x == line.len() - 1 || height < map[y][x + 1]) {
                Some(height + 1)
            } else {
                None
            }
        })
    }).sum::<u32>());
}
