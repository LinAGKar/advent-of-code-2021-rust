use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let mut pop = Vec::new();
    let mut add_table_0 = Vec::new();
    let mut add_table_1 = Vec::new();

    let mut get_num = |index| {
        lines.nth(index).unwrap().split_whitespace().nth(2).unwrap_or("0").parse::<i64>().unwrap_or(0)
    };

    for _ in 0..14 {
        pop.push(get_num(4) == 26);
        add_table_0.push(get_num(0));
        add_table_1.push(get_num(9));
        get_num(1);
    }

    let mut stack = Vec::new();
    let mut num = vec![0; 14];
    for i in 0..14 {
        if pop[i] {
            let (push_pos, push_add) = stack.pop().unwrap();
            let pop_sub = -add_table_0[i];
            let pushed_num = 1 + std::cmp::max(push_add, pop_sub);
            num[push_pos] = pushed_num - push_add;
            num[i] = pushed_num - pop_sub;
        } else {
            stack.push((i, add_table_1[i]));
        }
    }

    println!("{}", num.into_iter().fold(0, |acc, num| acc * 10 + num));
}
