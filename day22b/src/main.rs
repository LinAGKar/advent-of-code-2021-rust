use std::io::Read;

fn subtract_cuboid(a: [i32; 6], mut b: [i32; 6], result: &mut Vec<[i32; 6]>) {
    if (0..3).any(|i| {
        b[2 * i + 1] < a[2 * i] || b[2 * i] > a[2 * i + 1]
    }) {
        result.push(b);
    } else {
        for i in 0..3 {
            if b[2 * i] < a[2 * i] {
                let mut tmp = b;
                tmp[2 * i + 1] = a[2 * i] - 1;
                result.push(tmp);
            }
            if b[2 * i + 1] > a[2 * i + 1] {
                let mut tmp = b;
                tmp[2 * i] = a[2 * i + 1] + 1;
                result.push(tmp);
            }
            b[2 * i] = std::cmp::max(a[2 * i], b[2 * i]);
            b[2 * i + 1] = std::cmp::min(a[2 * i + 1], b[2 * i + 1]);
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let src_cuboids: Vec<_> = input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let on = parts.next().unwrap() == "on";
        let mut cuboid = [0; 6];
        for (src, tgt) in parts.next().unwrap().split(&[',', '=', '.'][..]).filter_map(|str| {
            str.parse().ok()
        }).zip(&mut cuboid) {
            *tgt = src;
        }
        (on, cuboid)
    }).collect();

    let mut enabled_cuboids = Vec::new();
    let mut new_enabled_cuboids = Vec::new();

    for (on, cuboid) in src_cuboids {
        new_enabled_cuboids.clear();
        for &enabled_cuboid in &enabled_cuboids {
            subtract_cuboid(cuboid, enabled_cuboid, &mut new_enabled_cuboids);
        }
        if on {
            new_enabled_cuboids.push(cuboid);
        }
        std::mem::swap(&mut enabled_cuboids, &mut new_enabled_cuboids);
    }

    println!("{}", enabled_cuboids.into_iter().map(|cuboid| {
        (0..3).map(|i| (cuboid[2 * i + 1] - cuboid[2 * i] + 1) as u64).product::<u64>()
    }).sum::<u64>());
}
