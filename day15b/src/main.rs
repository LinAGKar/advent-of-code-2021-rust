use std::collections::{BTreeSet,HashMap};
use std::io::Read;

fn h(pos: (i16, i16), goal: (i16, i16)) -> u32 {
    (pos.0 - goal.0).abs() as u32 + (pos.1 - goal.1).abs() as u32
}

fn a_star(grid: &Vec<Vec<u32>>, start: (i16, i16), goal: (i16, i16)) -> Option<u32> {
    let start_f_score = h(start, goal);

    let mut f_scores = HashMap::new();
    f_scores.insert(start, start_f_score);

    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut open_set = BTreeSet::new();
    open_set.insert((start_f_score, start));

    while let Some(&curr) = open_set.iter().next() {
        open_set.remove(&curr);

        let (f_score, pos) = curr;
        if pos == goal {
            return Some(f_score);
        }
        let (y, x) = pos;
        let g_score = g_scores[&pos];

        for (dy, dx) in [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ] {
            let (y, x) = (y + dy, x + dx);
            let danger = if let Some(&danger) = grid.get(y as usize).and_then(|row| row.get(x as usize)) {
                danger
            } else {
                continue;
            };
            let new_pos = (y, x);
            let tentative_g_score = g_score + danger;
            let old_g_score = g_scores.get(&new_pos).copied();
            if tentative_g_score < old_g_score.unwrap_or(u32::MAX) {
                if old_g_score.is_some() {
                    open_set.remove(&(f_scores[&new_pos], new_pos));
                }

                let new_f_score = tentative_g_score + h(new_pos, goal);
                f_scores.insert(new_pos, new_f_score);
                g_scores.insert(new_pos, tentative_g_score);
                open_set.insert((new_f_score, new_pos));
            }
        }
    }

    None
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let grid: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() as u32).collect()
    }).collect();
    let grid: Vec<Vec<_>> = (0..5).flat_map(|i| grid.iter().map(move |line| {
        (0..5).flat_map(|j| line.iter().map(move |&danger| (danger - 1 + i + j) % 9 + 1)).collect()
    })).collect();

    println!("{}", a_star(&grid, (0, 0), ((grid.len() - 1) as i16, (grid.last().unwrap().len() - 1) as i16)).unwrap());
}
