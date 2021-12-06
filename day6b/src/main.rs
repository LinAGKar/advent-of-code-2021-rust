use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut fish: VecDeque<_> = [0u64; 9].iter().copied().collect();

    for i in input.trim().split(',') {
        fish[i.parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..256 {
        let old_0 = fish.pop_front().unwrap();
        fish.push_back(old_0);
        fish[6] += old_0;
    }

    println!("{}", fish.into_iter().sum::<u64>());
}
