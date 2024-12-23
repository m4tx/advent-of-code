use advent::prelude::*;

type Input = HashMap<String, HashSet<String>>;

fn parse_input(input: &str) -> Input {
    let mut result = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split("-");
        let first = parts.next().unwrap().to_owned();
        let second = parts.next().unwrap().to_owned();

        result
            .entry(first.clone())
            .or_insert_with(HashSet::new)
            .insert(second.clone());
        result
            .entry(second.clone())
            .or_insert_with(HashSet::new)
            .insert(first.clone());
    }

    result
}

fn default_input() -> Input {
    parse_input(include_input!(2024 / 23))
}

fn part1(input: Input) -> i64 {
    let threes = get_threes(input);
    threes
        .iter()
        .filter(|[a, b, c]| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count() as i64
}

fn get_threes(input: Input) -> Vec<[String; 3]> {
    let mut result = Vec::new();
    let keys = input.keys().collect::<Vec<_>>();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            for k in j + 1..input.len() {
                let node_1 = keys[i].clone();
                let node_2 = keys[j].clone();
                let node_3 = keys[k].clone();

                if input[&node_1].contains(&node_2)
                    && input[&node_2].contains(&node_3)
                    && input[&node_3].contains(&node_1)
                {
                    result.push([node_1, node_2, node_3]);
                }
            }
        }
    }

    result
}

fn part2(input: Input) -> String {
    let mut largest = input
        .keys()
        .map(|node| largest_clique(&input, node))
        .max_by_key(|x| x.len())
        .unwrap();
    largest.sort();
    largest.join(",")
}

fn largest_clique<'a>(input: &'a Input, node: &'a str) -> Vec<&'a str> {
    let mut nodes = vec![node];
    for neighbor in input.keys() {
        if node == neighbor {
            continue;
        }

        if should_add_to_clique(input, &nodes, neighbor) {
            nodes.push(neighbor);
        }
    }

    nodes
}

fn should_add_to_clique<'a>(input: &'a Input, clique: &[&'a str], node: &'a str) -> bool {
    for member in clique {
        if !input[*member].contains(node) {
            return false;
        }
    }

    true
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[ignore]
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1306);
    assert_eq!(part2(input), "bd,dk,ir,ko,lk,nn,ob,pt,te,tl,uh,wj,yl");
}

#[test]
fn examples() {
    let input = parse_input(
        "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
    );
    assert_eq!(part1(input.clone()), 7);
    assert_eq!(part2(input), "co,de,ka,ta");
}
