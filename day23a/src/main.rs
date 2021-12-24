use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::Read;

#[derive(Default, PartialOrd, PartialEq, Eq, Ord, Clone, Copy, Debug, Hash)]
struct State {
    rooms: [[Option<u8>; 2]; 4],
    hallway: [Option<u8>; 7],
}

const HALLWAY_COLS: [i16; 7] = [0, 1, 3, 5, 7, 9, 10];
const ROOM_COLS: [i16; 4] = [2, 4, 6, 8];
const ROOM_HALLWAYS: [(usize, usize); 4] = [(1, 2), (2, 3), (3, 4), (4, 5)];
const COSTS: [i16; 4] = [1, 10, 100, 1000];

fn h(state: State) -> i16 {
    state.hallway.into_iter().enumerate().filter_map(|(n, spot)| {
        spot.map(|pod| {
            ((ROOM_COLS[pod as usize] - HALLWAY_COLS[n]).abs() + 1) * COSTS[pod as usize]
        })
    }).chain(state.rooms.into_iter().enumerate().flat_map(|(n, room)| {
        room.into_iter().enumerate().filter_map(move |(m, spot)| {
            spot.and_then(|pod| {
                if pod != n as u8 {
                    Some(((ROOM_COLS[pod as usize] - ROOM_COLS[n]).abs() + m as i16 + 2) * COSTS[pod as usize])
                } else {
                    None
                }
            })
        })
    })).chain(state.rooms.into_iter().enumerate().flat_map(|(n, room)| {
        room.into_iter().enumerate().filter_map(move |(m, spot)| {
            if spot == Some(n as u8) {
                None
            } else {
                Some((m as i16) * COSTS[n])
            }
        })
    })).sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut start: State = Default::default();
    for (n, line) in input.lines().skip(2).enumerate() {
        for (m, c) in line.chars().filter(|c| c.is_ascii_uppercase()).enumerate() {
            start.rooms[m][n] = Some(c as u8 - 'A' as u8);
        }
    }

    let mut open_set = BinaryHeap::new();
    open_set.push((Reverse(0), 0, start));

    let mut checked = HashMap::new();

    while let Some((_, cost, state)) = open_set.pop() {
        if checked.get(&state).map_or(false, |&old_cost| cost >= old_cost) {
            continue;
        }
        checked.insert(state, cost);

        if state.rooms.into_iter().enumerate().all(|(n, room)| {
            room.into_iter().all(|x| x == Some(n as u8))
        }) {
            println!("{}", cost);
            break;
        }

        for (n, room) in state.rooms.into_iter().enumerate() {
            if let Some(y) = if room.into_iter().any(|pod| pod.map_or(false, |pod| pod != n as u8)) {
                room.into_iter().enumerate().find_map(|(y, pod)| if pod.is_some() {
                    Some(y)
                } else {
                    None
                })
            } else {
                None
            } {
                for i in (0..).map(|i| ROOM_HALLWAYS[n].1 + i).take_while(|&i| {
                    state.hallway.get(i) == Some(&None)
                }).chain(
                    (0..).take_while(|&i| {
                        i <= ROOM_HALLWAYS[n].0 && state.hallway[ROOM_HALLWAYS[n].0 - i].is_none()
                    }).map(|i| ROOM_HALLWAYS[n].0 - i)
                ) {
                    let mut new_state = state;
                    std::mem::swap(&mut new_state.rooms[n][y], &mut new_state.hallway[i]);
                    let new_cost = cost + (
                        y as i16 + 1 + (ROOM_COLS[n] - HALLWAY_COLS[i]).abs()
                    ) * COSTS[state.rooms[n][y].unwrap() as usize];
                    let f_score = new_cost + h(new_state);

                    open_set.push((Reverse(f_score), new_cost, new_state));
                }
            }
        }

        for (x, spot) in state.hallway.into_iter().enumerate() {
            if let Some(pod) = spot {
                let room = state.rooms[pod as usize];
                if (room.into_iter().all(|spot| {
                    spot.map_or(true, |other_pod| other_pod == pod)
                })) && (if x >= ROOM_HALLWAYS[pod as usize].1 {
                    ROOM_HALLWAYS[pod as usize].1..x
                } else {
                    x + 1..ROOM_HALLWAYS[pod as usize].0 + 1
                }).all(|x| state.hallway[x].is_none()) {
                    let y = (0..room.len()).rev().find(|&spot| room[spot].is_none()).unwrap();
                    let mut new_state = state;
                    std::mem::swap(&mut new_state.rooms[pod as usize][y], &mut new_state.hallway[x]);
                    let new_cost = cost + (
                        y as i16 + 1 + (ROOM_COLS[pod as usize] - HALLWAY_COLS[x as usize]).abs()
                    ) * COSTS[pod as usize];
                    let f_score = new_cost + h(new_state);

                    open_set.push((Reverse(f_score), new_cost, new_state));
                }
            }
        }
    }
}
