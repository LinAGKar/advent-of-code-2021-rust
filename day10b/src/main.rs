use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut mapping = ['\0'; 128];
    mapping['(' as usize] = ')';
    mapping['[' as usize] = ']';
    mapping['{' as usize] = '}';
    mapping['<' as usize] = '>';

    let mut score = [0; 128];
    score[')' as usize] = 1;
    score[']' as usize] = 2;
    score['}' as usize] = 3;
    score['>' as usize] = 4;

    let mut stack = Vec::new();
    let mut scores: Vec<_> = input.lines().filter_map(|line| {
        stack.clear();

        for c in line.chars() {
            let right = mapping[c as usize];
            if right == '\0' {
                if c != stack.pop().unwrap() {
                    return None;
                }
            } else {
                stack.push(right);
            }
        }

        Some(stack.iter().zip(0..).map(|(&c, e)| score[c as usize] * 5u64.pow(e)).sum::<u64>())
    }).collect();

    let middle = scores.len() / 2;
    println!("{}", scores.select_nth_unstable(middle).1);
}
