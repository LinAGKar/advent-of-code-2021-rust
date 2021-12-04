use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut groups = input.split("\n\n");

    let drawn = groups.next().unwrap().split(',').map(|num| num.parse::<usize>().unwrap());
    let mut tiles_by_num = vec![Vec::new(); 100];

    let mut boards: Vec<Vec<Vec<_>>> = groups.enumerate().map(|(b, group)| {
        group.lines().enumerate().map(|(y, line)| {
            line.split_whitespace().enumerate().map(|(x, num)| {
                let num: usize = num.parse().unwrap();
                tiles_by_num[num].push((b, y, x));
                (false, num)
            }).collect()
        }).collect()
    }).collect();

    for i in drawn {
        for &(b, y, x) in &tiles_by_num[i] {
            boards[b][y][x].0 = true;

            if boards[b][y].iter().all(|&(marked, _)| marked) || boards[b].iter().all(|line| line[x].0) {
                println!("{}", boards[b].iter().flat_map(|line| line).filter_map(|&(marked, num)| {
                    if marked {
                        None
                    } else {
                        Some(num)
                    }
                }).sum::<usize>() * i);
                return;
            }
        }
    }
}
