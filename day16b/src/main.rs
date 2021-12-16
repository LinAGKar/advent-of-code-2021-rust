fn parse_sub_packets(start: usize, get_bits: &dyn Fn(usize, usize) -> u64) -> (Vec<u64>, usize) {
    let mut pos = start;
    let length_type = get_bits(pos, 1);
    let (max_len, max_packet_count) = if length_type == 0 {
        let len = get_bits(pos + 1, 15) as usize;
        pos += 16;
        (len, u32::MAX)
    } else {
        let count = get_bits(pos + 1, 11) as u32;
        pos += 12;
        (usize::MAX, count)
    };
    let start_pos = pos;

    let mut nums = Vec::new();

    for count in 1.. {
        let (num, new_pos) = parse_packet(pos, get_bits);
        pos = new_pos;
        nums.push(num);
        if count >= max_packet_count || pos - start_pos >= max_len {
            break;
        }
    }

    (nums, pos)
}

fn parse_packet(start: usize, get_bits: &dyn Fn(usize, usize) -> u64) -> (u64, usize) {
    let type_id = get_bits(start + 3, 3);

    match type_id {
        0 => {
            let (nums, pos) = parse_sub_packets(start + 6, get_bits);
            (nums.into_iter().sum(), pos)
        }

        1 => {
            let (nums, pos) = parse_sub_packets(start + 6, get_bits);
            (nums.into_iter().product(), pos)
        }

        2 => {
            let (nums, pos) = parse_sub_packets(start + 6, get_bits);
            (nums.into_iter().min().unwrap(), pos)
        }

        3 => {
            let (nums, pos) = parse_sub_packets(start + 6, get_bits);
            (nums.into_iter().max().unwrap(), pos)
        }

        4 => {
            let mut pos = start + 6;
            let mut num = 0;
            let mut keep_going = true;

            while keep_going {
                keep_going = get_bits(pos, 1) == 1;
                num = num << 4 | get_bits(pos + 1, 4);
                pos += 5;
            }

            (num, pos)
        }

        5 => {
            let (nums, pos) = parse_sub_packets(start + 6, get_bits);
            (if nums[0] > nums[1] { 1 } else { 0 }, pos)
        }

        6 => {
            let (nums, pos) = parse_sub_packets(start + 6, get_bits);
            (if nums[0] < nums[1] { 1 } else { 0 }, pos)
        }

        7 => {
            let (nums, pos) = parse_sub_packets(start + 6, get_bits);
            (if nums[0] == nums[1] { 1 } else { 0 }, pos)
        }

        _ => panic!(),
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
        data.iter().skip(start).take(len).fold(0, |acc, &i| acc << 1 | i as u64)
    };

    println!("{}", parse_packet(0, &get_bits).0);
}
