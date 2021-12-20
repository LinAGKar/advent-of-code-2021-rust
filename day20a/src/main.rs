use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let mapping: Vec<_> = lines.next().unwrap().chars().map(|c| if c == '#' { true } else { false }).collect();
    let image_template: Vec<Vec<_>> = lines.skip(1).map(|line| {
        line.chars().map(|c| if c == '#' { true } else { false }).collect()
    }).collect();

    const CYCLES: usize = 2;
    let mut height = image_template.len();
    let mut width = image_template[0].len();
    let mut surrounding_filled = false;
    let mut margin = CYCLES + 1;
    let full_width = width + margin * 2;
    let mut image = vec![false; (height + margin * 2) * full_width];
    for x in 0..width {
        for y in 0..height {
            image[(margin + y) * full_width + margin + x] = image_template[y][x];
        }
    }

    for _ in 0..CYCLES {
        let new_surrounding = if surrounding_filled { *mapping.last().unwrap() } else { mapping[0] };
        let mut new_image = vec![new_surrounding; image.len()];
        for x in margin - 1..margin + width + 1 {
            for y in margin - 1..margin + width + 1 {
                new_image[y * full_width + x] = mapping[(0..=2).flat_map(|dy| {
                    image.iter().skip((y + dy - 1) * full_width + x - 1).take(3)
                }).fold(0, |acc, &px| acc << 1 | if px { 1 } else { 0 })];
            }
        }

        margin -= 1;
        height += 2;
        width += 2;
        image = new_image;
        surrounding_filled = new_surrounding;
    }

    println!("{}", image.into_iter().filter(|&px| px).count());
}
