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
    score[')' as usize] = 3;
    score[']' as usize] = 57;
    score['}' as usize] = 1197;
    score['>' as usize] = 25137;

    let mut stack = Vec::new();
    println!("{}", input.lines().filter_map(|line| {
        stack.clear();

        for c in line.chars() {
            let right = mapping[c as usize];
            if right == '\0' {
                if c != stack.pop().unwrap() {
                    return Some(score[c as usize]);
                }
            } else {
                stack.push(right);
            }
        }

        None
    }).sum::<u32>());
}
