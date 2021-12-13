use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut parts = input.split("\n\n");

    let mut points: Vec<_> = parts.next().unwrap().lines().map(|line| {
        let mut nums = line.split(',').map(|num| num.parse::<i16>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    }).collect();

    let folds: Vec<_> = parts.next().unwrap().lines().map(|line| {
        let mut parts = line.split_whitespace().nth(2).unwrap().split('=');
        (parts.next().unwrap(), parts.next().unwrap().parse::<i16>().unwrap())
    }).collect();

    for fold in folds {
        for (x, y) in &mut points {
            let p = if fold.0 == "x" { x } else { y };
            if *p > fold.1 {
                *p = fold.1 - (*p - fold.1);
            }
        }
    }

    let points: std::collections::HashSet<_> = points.into_iter().collect();

    let mut min_x = i16::MAX;
    let mut min_y = i16::MAX;
    let mut max_x = i16::MIN;
    let mut max_y = i16::MIN;
    for &(x, y) in &points {
        min_x = std::cmp::min(x, min_x);
        min_y = std::cmp::min(y, min_y);
        max_x = std::cmp::max(x, max_x);
        max_y = std::cmp::max(y, max_y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", if points.contains(&(x, y)) { '█' } else { ' ' });
        }
        println!("");
    }
}
