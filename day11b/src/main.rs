use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut octopodes: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|c| (c.to_digit(10).unwrap() as u8, 0u8)).collect()
    }).collect();
    let octopus_count = octopodes.iter().flat_map(|row| row).count();

    let mut increasing = Vec::new();

    for t in 1.. {
        let mut count = 0;

        for y in 0..octopodes.len() {
            for x in 0..octopodes[y].len() {
                increasing.push((y, x));
                while let Some((y, x)) = increasing.pop() {
                    let (energy, last_flash) = &mut octopodes[y][x];
                    if *last_flash < t {
                        if *energy >= 9 {
                            *energy = 0;
                            *last_flash = t;
                            increasing.extend(
                                (-1..=1)
                                    .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
                                    .filter(|&pos| pos != (0, 0))
                                    .filter_map(|(dy, dx)| {
                                        let y = y as isize + dy;
                                        let x = x as isize + dx;

                                        if (dy, dx) == (0, 0) ||
                                           y < 0 || x < 0 ||
                                           y as usize >= octopodes.len() || x as usize >= octopodes[y as usize].len() {
                                            None
                                        } else {
                                            Some((y as usize, x as usize))
                                        }
                                    })
                            );
                            count += 1;
                        } else {
                            *energy += 1;
                        }
                    }
                }
            }
        }

        if count == octopus_count {
            println!("{}", t);
            break;
        }
    }
}
