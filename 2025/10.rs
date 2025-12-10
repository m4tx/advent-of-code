use advent::prelude::*;
use z3::Optimize;
use z3::ast::Int;

type Input = Vec<MachineInfo>;

#[derive(Debug, Clone)]
struct MachineInfo {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn parse_input(input: &str) -> Input {
    input.lines().map(parse_machine).collect()
}

fn parse_machine(line: &str) -> MachineInfo {
    let words: Vec<_> = line.split_whitespace().collect();

    let lights = words[0]
        .trim_matches(|c| c == '[' || c == ']')
        .chars()
        .map(|c| c == '#')
        .collect();

    let buttons = words[1..words.len() - 1]
        .iter()
        .map(|w| {
            w.trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let joltages = words[words.len() - 1]
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    MachineInfo {
        lights,
        buttons,
        joltages,
    }
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 10));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> usize {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct QueueState {
        light_states: Vec<bool>,
        steps: usize,
    }

    input
        .iter()
        .map(|machine| {
            let mut queue = VecDeque::new();
            queue.push_back(QueueState {
                light_states: vec![false; machine.lights.len()],
                steps: 0,
            });

            let mut seen_states = HashSet::new();

            while let Some(state) = queue.pop_front() {
                if seen_states.contains(&state.light_states) {
                    continue;
                }
                if machine.lights == state.light_states {
                    return state.steps;
                }

                seen_states.insert(state.light_states.clone());

                for button in &machine.buttons {
                    let mut new_lights = state.light_states.clone();
                    for &idx in button {
                        new_lights[idx] = !new_lights[idx];
                    }
                    queue.push_back(QueueState {
                        light_states: new_lights,
                        steps: state.steps + 1,
                    });
                }
            }

            panic!("no solution found for machine {:?}", machine);
        })
        .sum()
}

fn part2(input: Input) -> i64 {
    input
        .iter()
        .map(|machine| {
            let mut buttons = vec![];
            for (idx, _button) in machine.buttons.iter().enumerate() {
                buttons.push(Int::fresh_const(&format!("btn_{idx}")))
            }

            let solver = Optimize::new();
            for btn in &buttons {
                solver.assert(&btn.ge(0));
            }

            for (joltage_idx, &joltage) in machine.joltages.iter().enumerate() {
                let mut sum = Int::from_i64(0);
                for (btn_idx, button) in machine.buttons.iter().enumerate() {
                    if button.contains(&joltage_idx) {
                        sum += &buttons[btn_idx];
                    }
                }
                solver.assert(&sum.eq(joltage as i64));
            }

            solver.minimize(&buttons.iter().fold(Int::from_i64(0), |acc, b| acc + b));

            solver.check(&[]);
            let model = solver.get_model().unwrap();

            let vals: Vec<i64> = buttons
                .iter()
                .map(|b| model.eval(b, true).unwrap().as_i64().unwrap())
                .collect();
            vals.iter().sum::<i64>()
        })
        .sum()
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
        assert_eq!(part1(input.clone()), 473);
        assert_eq!(part2(input), 18681);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(part1(input.clone()), 7);
        assert_eq!(part2(input), 33);
    }
}
