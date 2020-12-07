use crate::util;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve(inputs: impl AsRef<str>) -> usize {
    let map: HashMap<String, HashSet<String>> = inputs
        .as_ref()
        .lines()
        .map(|l| {
            let (a, b) = l
                .replace("bags", "bag")
                .replace(
                    &[' ', '.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..],
                    "",
                )
                .split("contain")
                .map(|s| s.to_owned())
                .next_tuple()
                .unwrap();
            (
                a,
                b.split(",")
                    .filter(|&s| s != "nootherbag")
                    .map(|s| s.to_owned())
                    .collect(),
            )
        })
        .collect();

    let mut mem: HashMap<String, bool> = HashMap::new();
    mem.insert("shinygoldbag".to_owned(), true);

    map.iter()
        .filter(|(k, _)| populate_mem(k, &map, &mut mem))
        .count()
        - 1
}

fn populate_mem<'a, 'b, 'c: 'a + 'b>(
    bag: impl AsRef<str>,
    inputs: &'a HashMap<String, HashSet<impl AsRef<str>>>,
    mem: &'a mut HashMap<String, bool>,
) -> bool {
    if !mem.contains_key(bag.as_ref()) {
        let r = inputs[bag.as_ref()]
            .iter()
            .any(|s| populate_mem(s.clone(), inputs, mem));
        mem.insert(bag.as_ref().to_owned(), r);
    }
    mem[bag.as_ref()]
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"
        ),
        4
    );
}

#[test]
fn test() {
    let input = util::read_input("d7_handy_haversacks.txt");

    assert_eq!(solve(input), 148);
}
