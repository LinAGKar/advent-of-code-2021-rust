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

    let mut count = 0;

    for vy_0 in first_vy..=last_vy {
        // Time after which we return to y=0
        let y_0_time = if vy_0 <= 0 { 0 } else { 2 * vy_0 + 1 };

        'x_loop: for vx_0 in first_vx.. {
            let mut vx = std::cmp::max(0, vx_0 - y_0_time);
            let mut vy = vy_0 - y_0_time;

            // Take total distance probe will travel for given vx_0, and subtract the distance left to travel when at
            // current vx.
            let mut x = (vx_0.pow(2) + vx_0 - vx.pow(2) - vx) / 2;
            let mut y = 0;

            for t in y_0_time.. {
                if x > end_x {
                    if y > end_y || t == 1 {
                        // We overshot the target. Either the probe flew above it, or it flew past it in the first step
                        break 'x_loop;
                    } else {
                        // We missed the target
                        continue 'x_loop;
                    }
                } else if y < start_y {
                    // We missed the target
                    continue 'x_loop;
                } else if x >= start_x && y <= end_y {
                    // We hit the target
                    count += 1;
                    continue 'x_loop;
                }

                 x += vx;
                 y += vy;
                 vy -= 1;
                 if vx > 0 {
                    vx -= 1;
                 }
            }
        }
    }

    println!("{}", count);
}
