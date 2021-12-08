use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.lines().map(|line| {
        line
            .split_whitespace()
            .skip_while(|&word| word != "|")
            .skip(1)
            .filter(|&word| [2, 3, 4, 7].contains(&word.len()))
            .count()
    }).sum::<usize>());
}
