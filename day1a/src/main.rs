use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.lines().map(|line| line.parse().unwrap()).fold((0u16, u16::MAX), |(sum, prev), curr| {
        (if curr > prev { sum + 1 } else { sum }, curr)
    }).0);
}
