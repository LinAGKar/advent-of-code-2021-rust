fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<i32> = input.trim().split(',').map(|word| word.parse().unwrap()).collect();
    let mean = nums.iter().sum::<i32>() as f64 / nums.len() as f64;
    println!("{}", [mean.floor(), mean.ceil()].iter().map(|&mean| {
        nums.iter().map(|&num| {
            let d = (num - mean as i32).abs();
            d * (d + 1) / 2
        }).sum::<i32>()
    }).min().unwrap());
}
