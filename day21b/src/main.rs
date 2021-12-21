use std::io::Read;

#[derive(Clone, Copy)]
struct State {
    scores: [u8; 2],
    pos: [u16; 2],
    turn: u8,
}

fn to_index(state: State) -> usize {
    state.scores[0] as usize +
    state.scores[1] as usize * 30 +
    state.pos[0] as usize * (30 * 30) +
    state.pos[1] as usize * (30 * 30 * 10) +
    state.turn as usize * (30 * 30 * 10 * 10)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut state = State {
        scores: [0; 2],
        pos: [0; 2],
        turn: 0,
    };

    for (n, line) in input.lines().enumerate() {
        state.pos[n] = line.split_whitespace().nth(4).unwrap().parse::<u16>().unwrap() - 1;
    }

    let mut states = vec![(0u8, 0u64, Vec::<(usize, u8)>::new(), [false; 2]); 30 * 30 * 10 * 10 * 2];
    let mut checked_states = [false; 30 * 30 * 10 * 10 * 2];
    let mut to_check = vec![state];
    let start_index = to_index(state);
    checked_states[start_index] = true;

    const PER_DIE: u8 = 3;
    const DIES: u8 = 3;
    let mut rolls = [0; (PER_DIE * DIES) as usize - 2];
    for mut roll in 0..PER_DIE.pow(DIES as u32) {
        let mut score = 0;
        for _ in 0..DIES {
            score += roll % PER_DIE + 1;
            roll /= PER_DIE;
        }
        rolls[score as usize - DIES as usize] += 1;
    }

    while let Some(state) = to_check.pop() {
        let source_index = to_index(state);

        for (n, &weight) in rolls.iter().enumerate() {
            let roll = n as u16 + DIES as u16;

            let mut new_state = state;
            let turn = new_state.turn as usize;
            let pos = &mut new_state.pos[turn];
            *pos = (*pos + roll) % 10;
            new_state.scores[turn] += *pos as u8 + 1;
            new_state.turn = (new_state.turn + 1) % 2;
            let target_index = to_index(new_state);
            states[source_index].2.push((target_index, weight));
            states[target_index].0 += 1;

            if let Some(player) = (0..2).find(|&p| new_state.scores[p] >= 21) {
                states[target_index].3[player] = true;
            } else {
                if !checked_states[target_index] {
                    checked_states[target_index] = true;
                    to_check.push(new_state);
                }
            }
        }
    }

    states[start_index].1 = 1;
    let mut to_check = vec![start_index];
    let mut targets = Vec::new();

    let mut wins = [0; 2];

    while let Some(state) = to_check.pop() {
        let count = states[state].1;

        if let Some(player) = (0..2).find(|&p| states[state].3[p]) {
            wins[player] += count;
        } else {
            std::mem::swap(&mut targets, &mut states[state].2);

            for &(target, weight) in &targets {
                states[target].1 += weight as u64 * count;
                let target_count = &mut states[target].0;
                *target_count -= 1;
                if *target_count == 0 {
                    to_check.push(target);
                }
            }
        }
    }

    println!("{}", wins.iter().max().unwrap());
}
