fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split(&['=', '.', ',', ' ', '\n'][..]).map(|x| x.parse::<i16>());
    let start_x = parts.nth(3).unwrap().unwrap();
    let end_x = parts.nth(1).unwrap().unwrap();
    let start_y = parts.nth(2).unwrap().unwrap();
    let end_y = parts.nth(1).unwrap().unwrap();

    // Lowest possible starting y-velocity is where it hits the lowest row of the target in the first step.
    let first_vy = start_y;
    // Highest possible starting y-velocity is where it hits the lowest row of the target in the first step after
    // returning to y=0.
    let last_vy = -start_y - 1;

    // Lowest possible starting x-velocity is where the x-axis motion will halt as soon as possible after reaching the
    // target. Distance we will reach for a given velocity (vx), through and arithmetic progression sum, is
    // x = (vx^2 + vx) / 2. Targeting start_x (first column of target) gives start_x = (vx^2 + vx) / 2 <=>
    // vx = -1/2 +- sqrt((1/2)^2 + start_x). We then need to round up do end up inside the target.
    let first_vx = (-1.0 / 2.0 + (1.0 / 4.0 + 2.0 * start_x as f64).sqrt()).ceil() as i16;

    let mut vy_by_t = vec![Vec::new(); 2 * last_vy as usize + 3];

    for vy in first_vy..=last_vy {
        // Quadratic equation to find the time we reach a certain point
        // Use positive root. Negative root is extrapolating backwards in time
        let tmp = (2.0 * vy as f64 + 1.0) / 2.0;
        let first_t = (tmp + (tmp.powf(2.0) - 2.0 * end_y as f64).sqrt()).ceil() as usize;
        let last_t = (tmp + (tmp.powf(2.0) - 2.0 * start_y as f64).sqrt()).floor() as usize;

        for t in first_t..=last_t {
            vy_by_t[t].push(vy);
        }
    }

    let mut vx_by_vy = vec![0; (last_vy - first_vy + 1) as usize];

    let mut count = 0;

    for vx in first_vx..=end_x {
        // Quadratic equation to find the time we reach a certain point
        // Use subtraction root as that's when we reach the target the first time (both roots are positive if real)
        let tmp = (2.0 * vx as f64 + 1.0) / 2.0;
        let first_t = (tmp - (tmp.powf(2.0) - 2.0 * start_x as f64).sqrt()).ceil() as usize;
        let last_t = tmp - (tmp.powf(2.0) - 2.0 * end_x as f64).sqrt();
        // NaN (imaginary roots) means we never reach the end of the target area. Imaginary root for first_t would mean
        // we never enter target area, but we've eliminated those cases already by calculating first_vx.
        let last_t = if last_t.is_nan() { vy_by_t.len() - 1 } else { last_t.floor() as usize };

        for t in first_t..=last_t {
            for &vy in &vy_by_t[t] {
                if vx_by_vy[(vy - first_vy) as usize] != vx {
                    vx_by_vy[(vy - first_vy) as usize] = vx;
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
