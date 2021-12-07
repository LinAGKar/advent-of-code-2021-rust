fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut nums: Vec<i32> = input.trim().split(',').map(|word| word.parse().unwrap()).collect();
    let middle = nums.len() / 2;
    let median = *nums.select_nth_unstable(middle).1;
    println!("{}", nums.iter().map(|&num| (num - median).abs()).sum::<i32>());
}
