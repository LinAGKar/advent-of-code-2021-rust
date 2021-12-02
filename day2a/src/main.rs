use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut pos = 0;
    let mut depth = 0;
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let direction = words.next().unwrap();
        let distance: u16 = words.next().unwrap().parse().unwrap();

        match direction {
            "forward" => { pos += distance; }
            "down" => { depth += distance; }
            "up" => { depth -= distance; }
            _ => panic!(),
        }
    }

    println!("{}", pos as u32 * depth as u32);
}
