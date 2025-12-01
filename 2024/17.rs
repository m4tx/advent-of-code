use advent::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    opcodes: Rc<Vec<i64>>,
}

impl Input {
    fn read_combo(&self, combo: i64) -> i64 {
        match combo {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            val if (0i64..=3i64).contains(&combo) => val,
            _ => panic!("invalid combo address"),
        }
    }

    fn adv_combo(&self, rip: &mut usize) -> i64 {
        let combo = self.opcodes[*rip];
        *rip += 1;
        self.read_combo(combo)
    }

    fn adv_literal(&self, rip: &mut usize) -> i64 {
        let literal = self.opcodes[*rip];
        *rip += 1;
        literal
    }
}

fn parse_input(input: &str) -> Input {
    let reg_a_re = regex::Regex::new(r"^Register A: (\d+)$").unwrap();
    let reg_b_re = regex::Regex::new(r"^Register B: (\d+)$").unwrap();
    let reg_c_re = regex::Regex::new(r"^Register C: (\d+)$").unwrap();
    let prog_re = regex::Regex::new(r"^Program: ([0-9,]+)$").unwrap();

    let mut machine = Input {
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
        opcodes: Rc::new(Vec::new()),
    };
    for line in input.lines() {
        if let Some(captures) = reg_a_re.captures(line) {
            machine.reg_a = captures.get(1).unwrap().as_str().parse().unwrap();
        } else if let Some(captures) = reg_b_re.captures(line) {
            machine.reg_b = captures.get(1).unwrap().as_str().parse().unwrap();
        } else if let Some(captures) = reg_c_re.captures(line) {
            machine.reg_c = captures.get(1).unwrap().as_str().parse().unwrap();
        } else if let Some(captures) = prog_re.captures(line) {
            let opcodes = captures.get(1).unwrap().as_str();
            machine.opcodes = Rc::new(opcodes.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    machine
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2024 / 17));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> String {
    let mut output = Vec::new();
    simulate(input, &mut output);
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn simulate(mut input: Input, output: &mut Vec<i64>) {
    let mut rip = 0;
    let opcodes = &input.opcodes;

    while rip < opcodes.len() {
        let opcode = input.opcodes[rip];
        rip += 1;

        match opcode {
            0 => {
                // adv
                let arg1 = input.reg_a;
                let arg2 = 2i64.pow(input.adv_combo(&mut rip) as u32);
                input.reg_a = arg1 / arg2;
            }
            1 => {
                // bxl
                let arg1 = input.reg_b;
                let arg2 = input.adv_literal(&mut rip);
                input.reg_b = arg1 ^ arg2;
            }
            2 => {
                // bst
                let arg1 = input.adv_combo(&mut rip);
                input.reg_b = arg1 % 8;
            }
            3 => {
                // jnz
                let arg1 = input.adv_literal(&mut rip);
                if input.reg_a != 0 {
                    rip = arg1 as usize;
                }
            }
            4 => {
                // bxc
                let _arg1 = input.adv_literal(&mut rip);
                input.reg_b ^= input.reg_c;
            }
            5 => {
                // out
                let arg1 = input.adv_combo(&mut rip);
                output.push(arg1 % 8);
            }
            6 => {
                // bdv
                let arg1 = input.reg_a;
                let arg2 = 2i64.pow(input.adv_combo(&mut rip) as u32);
                input.reg_b = arg1 / arg2;
            }
            7 => {
                // cdv
                let arg1 = input.reg_a;
                let arg2 = 2i64.pow(input.adv_combo(&mut rip) as u32);
                input.reg_c = arg1 / arg2;
            }
            _ => {
                panic!("invalid opcode")
            }
        }
    }
}

fn part2(input: Input) -> i64 {
    calc_reg_a(input)
}

fn calc_reg_a(input: Input) -> i64 {
    let mut output = Vec::with_capacity(100);
    let mut values = vec![vec![]; 16];

    for opcode_index in 0..input.opcodes.len() {
        for i in 0..2i64.pow(11) {
            let new_number = i << (3 * opcode_index);
            simulate_with(input.clone(), &mut output, new_number);
            if output.len() > opcode_index && output[opcode_index] == input.opcodes[opcode_index] {
                values[opcode_index].push(i);
            }
        }
    }

    match check_values(input.clone(), &mut output, &values, 0, 0) {
        Some(val) => val,
        None => panic!("couldn't find a solution"),
    }
}

fn check_values(
    input: Input,
    output: &mut Vec<i64>,
    values: &[Vec<i64>],
    index: usize,
    number: i64,
) -> Option<i64> {
    if index >= values.len() {
        return Some(number);
    }

    for value in &values[index] {
        let reg_a = value << (3 * index) | number;
        if reg_a > 8i64.pow(values.len() as u32) {
            continue;
        }

        simulate_with(input.clone(), output, reg_a);

        if output.len() > index
            && output[0..index + 1] == input.opcodes[0..index + 1]
            && output.len() <= input.opcodes.len()
            && let Some(val) = check_values(input.clone(), output, values, index + 1, reg_a)
        {
            return Some(val);
        }
    }

    None
}

fn simulate_with(mut input: Input, output: &mut Vec<i64>, reg_a: i64) {
    output.clear();
    input.reg_a = reg_a;
    simulate(input, output);
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn default() {
        let input = default_input();
        assert_eq!(part1(input.clone()), "7,5,4,3,4,5,3,4,6");
        assert_eq!(part2(input), 164278899142333);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        );
        assert_eq!(part1(input.clone()), "4,6,3,5,6,3,5,2,1,0");
    }
}
