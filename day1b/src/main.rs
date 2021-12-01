use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.lines().map(|line| line.parse().unwrap()).fold(
        (VecDeque::<u16>::with_capacity(3), 0, 0u16),
        |(mut window, window_sum, mut count), new| {
            let mut new_window_sum = window_sum + new;
            if window.len() == 3 {
                new_window_sum -= window[0];
                window.pop_front();
                if new_window_sum > window_sum {
                    count += 1;
                }
            }
            window.push_back(new);
            (window, new_window_sum, count)
        },
    ).2);
}
