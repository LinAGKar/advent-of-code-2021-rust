use std::io::Read;

#[derive(Clone, Copy)]
enum Ref {
    Var(usize),
    Num(i64),
}

#[derive(Clone, Copy)]
enum Instr {
    Inp(Ref),
    Add(Ref, Ref),
    Mul(Ref, Ref),
    Div(Ref, Ref),
    Mod(Ref, Ref),
    Eql(Ref, Ref),
}

fn parse_ref(string: &str) -> Ref {
    string.parse().map_or_else(|_| {
        Ref::Var(string.chars().next().unwrap() as usize - 'w' as usize)
    }, |num| Ref::Num(num))
}

fn get_val(vars: [i64; 4], reference: Ref) -> i64 {
    match reference {
        Ref::Var(pos) => vars[pos],
        Ref::Num(num) => num,
    }
}

fn set_val(vars: &mut [i64; 4], reference: Ref, val: i64) {
    match reference {
        Ref::Var(pos) => { vars[pos] = val },
        Ref::Num(_) => panic!(),
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let monad: Vec<_> = input.lines().map(|line| {
        let mut words = line.split_whitespace();

        match words.next().unwrap() {
            "inp" => {
                Instr::Inp(parse_ref(words.next().unwrap()))
            }

            "add" => {
                Instr::Add(parse_ref(words.next().unwrap()), parse_ref(words.next().unwrap()))
            }

            "mul" => {
                Instr::Mul(parse_ref(words.next().unwrap()), parse_ref(words.next().unwrap()))
            }

            "div" => {
                Instr::Div(parse_ref(words.next().unwrap()), parse_ref(words.next().unwrap()))
            }

            "mod" => {
                Instr::Mod(parse_ref(words.next().unwrap()), parse_ref(words.next().unwrap()))
            }

            "eql" => {
                Instr::Eql(parse_ref(words.next().unwrap()), parse_ref(words.next().unwrap()))
            }

            _ => panic!(),
        }
    }).collect();

    for i in 0..9i64.pow(14) {
        let mut vars = [0; 4];
        let mut pos = 13;
        let mut model_number = 0;

        for &j in &monad {
            match j {
                Instr::Inp(a) => {
                    let num = (i / 9i64.pow(pos)) % 9 + 1;
                    model_number = model_number * 10 + num;
                    pos -= 1;
                    set_val(&mut vars, a, num);
                }

                Instr::Add(a, b) => {
                    let val = get_val(vars, a) + get_val(vars, b);
                    set_val(&mut vars, a, val);
                }

                Instr::Mul(a, b) => {
                    let val = get_val(vars, a) * get_val(vars, b);
                    set_val(&mut vars, a, val);
                }

                Instr::Div(a, b) => {
                    let val = get_val(vars, a) / get_val(vars, b);
                    set_val(&mut vars, a, val);
                }

                Instr::Mod(a, b) => {
                    let val = get_val(vars, a) % get_val(vars, b);
                    set_val(&mut vars, a, val);
                }

                Instr::Eql(a, b) => {
                    let val = if get_val(vars, a) == get_val(vars, b) { 1 } else { 0 };
                    set_val(&mut vars, a, val);
                }
            }
        }

        if vars[3] == 0 {
            println!("{}", model_number);
            break;
        }
    }
}
