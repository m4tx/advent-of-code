use advent::prelude::*;

#[derive(Debug, Clone)]
struct Input {
    reverse_deps: HashMap<String, Vec<String>>,
}

fn parse_input(input: &str) -> Input {
    let mut reverse_deps: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(':');
        let name = parts.next().unwrap().trim();
        let rest = parts.next().unwrap().trim();
        let dependencies: Vec<String> = rest.split_whitespace().map(|s| s.to_owned()).collect();

        for dep in &dependencies {
            reverse_deps
                .entry(dep.to_owned())
                .or_default()
                .push(name.to_owned());
        }
    }

    Input { reverse_deps }
}

fn default_input() -> Input {
    #[cfg(feature = "default-inputs")]
    return parse_input(include_input!(2025 / 11));
    #[cfg(not(feature = "default-inputs"))]
    panic!("default-inputs feature not enabled");
}

fn part1(input: Input) -> i64 {
    let mut paths: HashMap<String, i64> = HashMap::with_capacity(input.reverse_deps.len());
    paths.insert("you".to_owned(), 1);
    get_num_paths(&input, &mut paths, "out")
}

fn get_num_paths(input: &Input, paths: &mut HashMap<String, i64>, node: &str) -> i64 {
    if let Some(&count) = paths.get(node) {
        return count;
    }

    let count = if let Some(dependents) = input.reverse_deps.get(node) {
        dependents
            .iter()
            .map(|dep| get_num_paths(input, paths, dep))
            .sum()
    } else {
        0
    };

    paths.insert(node.to_owned(), count);
    count
}

fn part2(input: Input) -> i64 {
    let mut paths: HashMap<VisitedKey, i64> = HashMap::with_capacity(input.reverse_deps.len() * 4);
    paths.insert(VisitedKey::new("svr", false, false), 1);
    get_num_paths_2(&input, &mut paths, VisitedKey::new("out", true, true))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VisitedKey {
    name: String,
    visited_dac: bool,
    visited_fft: bool,
}

impl VisitedKey {
    pub fn new(node: impl Into<String>, visited_dac: bool, visited_fft: bool) -> Self {
        Self {
            name: node.into(),
            visited_dac,
            visited_fft,
        }
    }
}

fn get_num_paths_2(input: &Input, paths: &mut HashMap<VisitedKey, i64>, node: VisitedKey) -> i64 {
    if let Some(&count) = paths.get(&node) {
        return count;
    }

    let count = if let Some(dependents) = input.reverse_deps.get(&node.name) {
        let mut visited_flags = HashSet::new();
        visited_flags.insert((true, true));
        if !node.visited_dac || node.name == "dac" {
            visited_flags.insert((false, true));
            visited_flags.insert((false, node.visited_fft));
        }
        if !node.visited_fft || node.name == "fft" {
            visited_flags.insert((true, false));
            visited_flags.insert((node.visited_dac, false));
        }

        let mut total = 0;
        for (v_dac, v_fft) in visited_flags {
            for dep in dependents {
                total += get_num_paths_2(input, paths, VisitedKey::new(dep, v_dac, v_fft));
            }
        }
        total
    } else {
        0
    };

    paths.insert(node, count);
    count
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
        assert_eq!(part1(input.clone()), 764);
        assert_eq!(part2(input), 462444153119850);
    }

    #[test]
    fn examples() {
        let input = parse_input(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
",
        );
        assert_eq!(part1(input), 5);

        let input = parse_input(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
",
        );
        assert_eq!(part2(input), 2);
    }
}
