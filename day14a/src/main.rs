use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut elements = vec![None; 26];
    let mut element_count = 0;
    for c in input.chars() {
        if c.is_ascii_uppercase() {
            let letter = &mut elements[c as usize - 'A' as usize];
            if letter.is_none() {
                *letter = Some(element_count);
                element_count += 1;
            }
        }
    }

    let mut lines = input.lines();
    let template: Vec<_> = lines.next().unwrap().chars().map(|c| {
        elements[c as usize - 'A' as usize].unwrap()
    }).collect();

    let mut counts = vec![0u16; element_count];
    for &element in &template {
        counts[element] += 1;
    }

    let mut pairs = vec![0; element_count.pow(2)];
    for pair in template.windows(2) {
        pairs[pair[0] * element_count + pair[1]] += 1;
    }

    let mut rules = vec![(0, 0, 0); element_count.pow(2)];
    for line in lines.skip(1) {
        let rule_elems: Vec<_> = line.chars().filter_map(|c| {
            if c.is_ascii_uppercase() {
                Some(elements[c as usize - 'A' as usize].unwrap())
            } else {
                None
            }
        }).collect();

        rules[rule_elems[0] * element_count + rule_elems[1]] = (
            rule_elems[0] * element_count + rule_elems[2], rule_elems[2] * element_count + rule_elems[1], rule_elems[2],
        );
    }

    let mut new_pairs = vec![0; element_count.pow(2)];

    for _ in 0..10 {
        for i in &mut new_pairs {
            *i = 0;
        }

        for (&count, &rule) in pairs.iter().zip(&rules) {
            new_pairs[rule.0] += count;
            new_pairs[rule.1] += count;
            counts[rule.2] += count;
        }

        std::mem::swap(&mut pairs, &mut new_pairs);
    }

    println!("{}", counts.iter().max().unwrap() - counts.iter().filter(|&&count| count > 0).min().unwrap());
}
