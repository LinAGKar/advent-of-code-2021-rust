fn parse_packet(start: usize, get_bits: &dyn Fn(usize, usize) -> u32) -> (u32, usize) {
    let version = get_bits(start, 3);
    let type_id = get_bits(start + 3, 3);

    match type_id {
        4 => {
            let mut pos = start + 6;
            let mut num = 0;
            let mut keep_going = true;

            while keep_going {
                keep_going = get_bits(pos, 1) == 1;
                num = num << 4 | get_bits(pos + 1, 4);
                pos += 5;
            }

            (version, pos)
        }

        _ => {
            let mut pos = start + 6;
            let length_type = get_bits(pos, 1);
            let (max_len, max_packet_count) = if length_type == 0 {
                let len = get_bits(pos + 1, 15) as usize;
                pos += 16;
                (len, u32::MAX)
            } else {
                let count = get_bits(pos + 1, 11);
                pos += 12;
                (usize::MAX, count)
            };
            let start_pos = pos;

            let mut version_sum = version;

            for count in 1.. {
                let (sub_version_sum, new_pos) = parse_packet(pos, get_bits);
                pos = new_pos;
                version_sum += sub_version_sum;
                if count >= max_packet_count || pos - start_pos >= max_len {
                    break;
                }
            }

            (version_sum, pos)
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let data: Vec<_> = input.trim().chars().flat_map(|c| {
        let num = c.to_digit(16).unwrap();
        (0..4).map(move |i| (num >> 3 - i & 0b1) as u8)
    }).collect();

    let get_bits = move |start: usize, len: usize| {
        data.iter().skip(start).take(len).fold(0, |acc, &i| acc << 1 | i as u32)
    };

    println!("{}", parse_packet(0, &get_bits).0);
}
