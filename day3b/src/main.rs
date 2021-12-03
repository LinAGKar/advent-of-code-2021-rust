use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", (0..2).map(|i| {
        let mut numbers: Vec<Vec<u8>> = input.lines().map(|line| {
            line.chars().map(|digit| digit as u8 - '0' as u8).collect()
        }).collect();

        for j in 0.. {
            let mut counts = [0, 0];

            for number in &numbers {
                counts[number[j] as usize] += 1;
            }

            let digit = (0..2).max_by_key(|&k| counts[k]).unwrap() as u8 ^ i;

            numbers = numbers.into_iter().filter(|number| number[j] == digit).collect();

            if numbers.len() == 1 {
                return numbers[0].iter().fold(0, |num, &digit| (num << 1) | digit as u32);
            }
        }

        panic!();
    }).product::<u32>());
}
