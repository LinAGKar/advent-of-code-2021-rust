use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut parts = input.split("\n\n");

    let points: Vec<_> = parts.next().unwrap().lines().map(|line| {
        let mut nums = line.split(',').map(|num| num.parse::<i16>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    }).collect();

    let fold = parts.next().unwrap().lines().map(|line| {
        let mut parts = line.split_whitespace().nth(2).unwrap().split('=');
        (parts.next().unwrap(), parts.next().unwrap().parse::<i16>().unwrap())
    }).next().unwrap();

    let points: std::collections::HashSet<_> = points.iter().map(|&(mut pos)| {
        let p = if fold.0 == "x" { &mut pos.0 } else { &mut pos.1 };
        if *p > fold.1 {
            *p = fold.1 - (*p - fold.1);
        }
        pos
    }).collect();

    println!("{:?}", points.len());
}
