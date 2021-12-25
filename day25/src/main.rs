use std::io::Read;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    East,
    South,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut east = Vec::new();
    let mut south = Vec::new();
    let mut map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            (0, match c {
                '>' => {
                    east.push((y, x));
                    Tile::East
                }

                'v' => {
                    south.push((y, x));
                    Tile::South
                }

                '.' => Tile::Empty,
                _ => panic!(),
            })
        }).collect()
    }).collect();

    for i in 1.. {
        let mut moved = false;

        for (n, (list, tile_type, dy, dx)) in [
            (&mut east, Tile::East, 0, 1),
            (&mut south, Tile::South, 1, 0),
        ].into_iter().enumerate() {
            let time = 2 * i + n;

            for pos in list {
                let (y, x) = *pos;

                let (mut new_y, mut new_x) = (y + dy, x + dx);
                if new_y >= map.len() {
                    new_y = 0;
                }
                if new_x >= map[new_y].len() {
                    new_x = 0;
                }

                map[y][x].0 = time;
                if map[new_y][new_x].1 == Tile::Empty && map[new_y][new_x].0 < time {
                    moved = true;
                    map[y][x].1 = Tile::Empty;
                    map[new_y][new_x].1 = tile_type;
                    *pos = (new_y, new_x);
                }
            }
        }

        if !moved {
            println!("{}", i);
            break;
        }
    }
}
