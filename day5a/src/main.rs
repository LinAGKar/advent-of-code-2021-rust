use std::io::Read;
use std::iter::Iterator;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut covered = vec![[0u8; 1000]; 1000];
    let mut count = 0;

    let mut add_poses = |poses: &mut dyn Iterator<Item=(usize, usize)>| {
        for pos in poses {
            if covered[pos.0][pos.1] == 1 {
                count += 1;
            }
            covered[pos.0][pos.1] += 1;
        }
    };

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let words = [words.next().unwrap(), words.nth(1).unwrap()];
        let mut ends = words.iter().map(|string| {
            let mut nums = string.split(',').map(|word| word.parse().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        });
        let ends = [ends.next().unwrap(), ends.next().unwrap()];
        let ends = [ends.iter().min().unwrap(), ends.iter().max().unwrap()];

        if ends[0].0 == ends[1].0 {
            add_poses(&mut (ends[0].1..=ends[1].1).map(|y| (ends[0].0, y)));
        } else if ends[0].1 == ends[1].1 {
            add_poses(&mut (ends[0].0..=ends[1].0).map(|x| (x, ends[0].1)));
        }
    }

    println!("{}", count);
}
