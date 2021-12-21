use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut players: Vec<_> = input.lines().map(|line| {
        (line.split_whitespace().nth(4).unwrap().parse::<u16>().unwrap() - 1, 0)
    }).collect();

    let mut die_val = 99;
    let mut roll_count = 0;

    'outer: loop {
        for (n, (pos, score)) in players.iter_mut().enumerate() {
            let roll = (0..3).map(|_| {
                roll_count += 1;
                die_val = (die_val + 1) % 100;
                die_val + 1
            }).sum::<u16>();

            let prev = *pos;
            *pos = (*pos + roll) % 10;
            println!("Player {} roll {} move from {} to {}", n + 1, roll, prev + 1, *pos + 1);
            *score += *pos + 1;
            if *score >= 1000 {
                break 'outer;
            }
        }
    }

    println!("{} {}", players.iter().map(|&(_, score)| score as u32).min().unwrap(), roll_count);
    println!("{}", players.into_iter().map(|(_, score)| score as u32).min().unwrap() * roll_count);
}
