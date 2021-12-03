use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut counts = Vec::new();
    for number in input.lines() {
        counts.resize(number.len(), [0, 0]);
        for (counts, digit) in counts.iter_mut().zip(number.chars()) {
            counts[digit as usize - '0' as usize] += 1;
        }
    }

    let mut gamma: u32 = 0;
    let mask = !(!0 << counts.len());
    for counts in counts.into_iter() {
        gamma <<= 1;

        if counts[1] > counts[0] {
            gamma |= 1;
        }
    }

    println!("{}", gamma * (mask ^ gamma));
}
