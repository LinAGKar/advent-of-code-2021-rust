use std::collections::HashMap;
use std::io::Read;

#[derive(Debug)]
struct Cave {
    connected: Vec<usize>,
    large: bool,
}

fn count_paths(caves: &Vec<Cave>, visited: &mut Vec<bool>, pos: usize, end: usize) -> u16 {
    let cave = &caves[pos];

    if pos == end {
        1
    } else if visited[pos] && !cave.large {
        0
    } else {
        visited[pos] = true;
        let paths = cave.connected.iter().map(|&new_pos| {
            count_paths(caves, visited, new_pos, end)
        }).sum();
        visited[pos] = false;
        paths
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut cave_indexes = HashMap::new();
    let mut caves = Vec::new();

    for line in input.lines() {
        let words: Vec<_> = line.split('-').collect();
        for &word in &words {
            if !cave_indexes.contains_key(word) {
                cave_indexes.insert(word, caves.len());

                caves.push(Cave {
                    connected: Vec::new(),
                    large: word.chars().next().unwrap().is_ascii_uppercase(),
                });
            }
        }

        let a = cave_indexes[words[0]];
        let b = cave_indexes[words[1]];
        caves[a].connected.push(b);
        caves[b].connected.push(a);
    }

    let start = cave_indexes["start"];
    let end = cave_indexes["end"];

    println!("{}", count_paths(&caves, &mut vec![false; caves.len()], start, end));
}
