use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let dummy = HashSet::new();

    println!("{}", input.lines().map(|line| {
        let mut parts = line.split('|');

        let inp_digits: Vec<HashSet<char>> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|digit| digit.chars().collect())
            .collect();

        let mut digits = [&dummy; 10];
        let mut segments = ['\0'; 7];

        digits[1] = &inp_digits.iter().find(|&digit| digit.len() == 2).unwrap();
        digits[4] = &inp_digits.iter().find(|&digit| digit.len() == 4).unwrap();
        digits[7] = &inp_digits.iter().find(|&digit| digit.len() == 3).unwrap();
        digits[8] = &inp_digits.iter().find(|&digit| digit.len() == 7).unwrap();
        segments[0] = *digits[7].iter().find(|&sig| !digits[1].contains(sig)).unwrap();
        segments[1] = *digits[4].iter().find(|&sig| {
            !digits[1].contains(sig) && inp_digits.iter().all(|digit| digit.len() != 6 || digit.contains(sig))
        }).unwrap();
        digits[5] = &inp_digits.iter().find(|&digit| digit.len() == 5 && digit.contains(&segments[1])).unwrap();
        segments[2] = *digits[1].iter().find(|&sig| !digits[5].contains(sig)).unwrap();
        segments[5] = *digits[1].iter().find(|&sig| digits[5].contains(sig)).unwrap();
        digits[2] = &inp_digits.iter().find(|&digit| digit.len() == 5 && !digit.contains(&segments[5])).unwrap();
        digits[3] = &inp_digits.iter().find(|&digit| {
            digit.len() == 5 && digit.contains(&segments[2]) && digit.contains(&segments[5])
        }).unwrap();
        digits[6] = &inp_digits.iter().find(|&digit| digit.len() == 6 && !digit.contains(&segments[2])).unwrap();
        segments[4] = *digits[2].iter().find(|&sig| !digits[3].contains(sig)).unwrap();
        digits[9] = &inp_digits.iter().find(|&digit| digit.len() == 6 && !digit.contains(&segments[4])).unwrap();
        digits[0] = &inp_digits.iter().find(|&digit| {
            digit.len() == 6 && digit.contains(&segments[4]) && digit != digits[6]
        }).unwrap();

        parts.next().unwrap().split_whitespace().fold(0, |acc, digit| {
            let digit: HashSet<_> = digit.chars().collect();

            acc * 10 + digits.iter().enumerate().find_map(|(n, &ref_digit)| if digit == *ref_digit {
                Some(n)
            } else {
                None
            }).unwrap()
        })
    }).sum::<usize>());
}
