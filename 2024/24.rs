use advent::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    inputs: HashMap<String, bool>,
    gates: Vec<Gate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Gate {
    left: String,
    right: String,
    output: String,
    op: Operator,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn apply(&self, left: bool, right: bool) -> bool {
        match self {
            Operator::And => left && right,
            Operator::Or => left || right,
            Operator::Xor => left ^ right,
        }
    }
}

fn parse_input(input: &str) -> Input {
    enum Mode {
        Inputs,
        Operations,
    }

    let mut mode = Mode::Inputs;
    let mut result = Input {
        inputs: HashMap::new(),
        gates: Vec::new(),
    };

    for line in input.lines() {
        if line.is_empty() {
            mode = Mode::Operations;
            continue;
        }

        match mode {
            Mode::Inputs => {
                let mut it = line.split_whitespace();
                let name = it.next().unwrap().trim_end_matches(':').to_string();
                let value = it.next().unwrap().parse::<i32>().unwrap();
                result.inputs.insert(name, value != 0);
            }
            Mode::Operations => {
                let mut it = line.split_whitespace();
                let left = it.next().unwrap().to_string();
                let op = it.next().unwrap();
                let right = it.next().unwrap().to_string();
                let _ = it.next();
                let output = it.next().unwrap().to_string();

                let op = match op {
                    "AND" => Operator::And,
                    "OR" => Operator::Or,
                    "XOR" => Operator::Xor,
                    _ => unreachable!(),
                };

                result.gates.push(Gate {
                    left,
                    right,
                    output,
                    op,
                });
            }
        }
    }

    result
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 24))
}

fn part1(input: Input) -> i64 {
    let values = calc(input);

    get_num(&values, "z")
}

fn calc(mut input: Input) -> HashMap<String, bool> {
    let mut values = HashMap::new();
    for value in input.inputs {
        values.insert(value.0, value.1);
    }

    let mut operations = std::mem::take(&mut input.gates);
    while !operations.is_empty() {
        let mut new_operations = Vec::new();

        for operation in std::mem::take(&mut operations) {
            let left = values.get(&operation.left);
            let right = values.get(&operation.right);

            match (left, right) {
                (Some(&left), Some(&right)) => {
                    let result = operation.op.apply(left, right);
                    values.insert(operation.output, result);
                }
                _ => {
                    new_operations.push(operation);
                }
            }

            operations = new_operations.clone();
        }
    }
    values
}

fn get_num(values: &HashMap<String, bool>, prefix: &str) -> i64 {
    let mut result = 0;
    for i in 0..100 {
        if let Some(value) = values.get(&format!("{prefix}{i:02}")) {
            if *value {
                result |= 1 << i;
            }
        } else {
            break;
        }
    }
    result
}

fn part2(mut input: Input) -> i64 {
    // doesn't really produce the answer; can be used to manually identify swapped gates
    let result = calc(input.clone());
    let x = get_num(&result, "x");
    let y = get_num(&result, "y");
    let z = get_num(&result, "z");

    for i in 0..45 {
        if (z >> i) & 1 != ((x + y) >> i) & 1 {
            println!("mismatch at {i}");
        }
    }

    println!("    z = {:b}", z);
    println!("x + y = {:b}", x + y);

    for _ in 0..50 {
        rename_matching(
            &mut input,
            |gate| {
                gate.left.starts_with("carry")
                    && gate.right.starts_with("sum")
                    && &gate.left[5..] == "00"
            },
            "carry",
        );
        rename_matching(
            &mut input,
            |gate| {
                gate.left[1..] == gate.right[1..]
                    && gate.op == Operator::Xor
                    && (gate.left.starts_with("x") || gate.left.starts_with("y"))
            },
            "sum",
        );
        rename_matching(
            &mut input,
            |gate| {
                &gate.left[1..] != "00"
                    && gate.left[1..] == gate.right[1..]
                    && gate.op == Operator::And
                    && (gate.left.starts_with("x") || gate.left.starts_with("y"))
            },
            "carry_two_",
        );
        rename_matching(
            &mut input,
            |gate| {
                &gate.left[1..] == "00"
                    && &gate.right[1..] == "00"
                    && gate.op == Operator::And
                    && (gate.left.starts_with("x") || gate.left.starts_with("y"))
            },
            "carry",
        );
        rename_matching(
            &mut input,
            |gate| {
                gate.left.starts_with("carry")
                    && gate.right.starts_with("sum")
                    && gate.op == Operator::And
            },
            "carry_one_",
        );
        rename_matching(
            &mut input,
            |gate| {
                gate.left.starts_with("sum")
                    && gate.right.starts_with("carry")
                    && gate.op == Operator::And
            },
            "carry_one_",
        );
        rename_matching(
            &mut input,
            |gate| {
                gate.left.starts_with("carry_two_")
                    && gate.right.starts_with("carry_one_")
                    && gate.op == Operator::Or
            },
            "carry",
        );
        rename_matching(
            &mut input,
            |gate| {
                gate.left.starts_with("carry_one_")
                    && gate.right.starts_with("carry_two_")
                    && gate.op == Operator::Or
            },
            "carry",
        );
    }

    print_gate(&input, "z36", 4);

    0
}

fn rename_matching<F: Fn(&Gate) -> bool>(input: &mut Input, predicate: F, new_prefix: &str) {
    for gate in input.gates.clone() {
        if predicate(&gate) {
            let digits = gate
                .left
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            rename_gate(input, &gate.output, &format!("{}{}", new_prefix, digits));
        }
    }
}

fn print_gate(input: &Input, gate_name: &str, depth: i32) {
    if depth == 0 {
        return;
    }
    for gate in &input.gates {
        if gate.output == gate_name {
            println!("{:?}", gate);

            print_gate(input, &gate.left, depth - 1);
            print_gate(input, &gate.right, depth - 1);
        }
    }
}

fn rename_gate(input: &mut Input, old_name: &str, new_name: &str) {
    if old_name == new_name {
        return;
    }

    println!("renaming {} to {}", old_name, new_name);
    for operation in &mut input.gates {
        if operation.left == old_name {
            operation.left = new_name.to_string();
        }
        if operation.right == old_name {
            operation.right = new_name.to_string();
        }
        if operation.output == old_name {
            operation.output = new_name.to_string();
        }
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 55661564905190);
}

#[test]
fn examples() {
    let input = parse_input(
        "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
    );
    assert_eq!(part1(input.clone()), 4);
}
