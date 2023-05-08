use evalexpr::*;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
#[derive(Debug, Default)]
struct Monkey {
    id: i32,
    items: VecDeque<i32>,
    op: String,
    div: i32,
    if_true: i32,
    if_false: i32,
    inspect: i32,
}

impl Monkey {
    fn new(
        id: i32,
        items: VecDeque<i32>,
        op: String,
        div: i32,
        if_true: i32,
        if_false: i32,
    ) -> Self {
        Monkey {
            id,
            items,
            op,
            div,
            if_true,
            if_false,
            ..Default::default()
        }
    }

    fn operation(&mut self) -> i32 {
        let item = self.items.pop_front().unwrap();
        let code = self.op.replace("old", &item.to_string());
        eval(&code).unwrap().as_int().unwrap() as i32
    }
}

#[derive(Debug, Default)]
struct MonkeyGroup {
    group: Vec<Monkey>,
}

impl MonkeyGroup {
    fn new() -> Self {
        Default::default()
    }

    fn round() {}
}

fn parse_monkey(s: &str) -> Monkey {
    let mut lines = s.lines();
    let id = lines
        .next()
        .unwrap()
        .split(&[' ', ':'][..])
        .collect::<Vec<_>>()[1]
        .parse::<i32>()
        .unwrap();
    let items = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<VecDeque<_>>();
    let op = lines
        .next()
        .unwrap()
        .split("= ")
        .nth(1)
        .unwrap()
        .to_string();
    let div = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let if_true = lines
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let if_false = lines
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let monkey = Monkey::new(id, items, op, div, if_true, if_false);
    monkey
}

fn parse(s: String) -> MonkeyGroup {
    let mut monkeys = MonkeyGroup::new();

    let m = s.split("\n\n").collect::<Vec<_>>();
    m.iter().for_each(|&m| monkeys.group.push(parse_monkey(m)));
    monkeys
}
fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/test.txt");

    let content = fs::read_to_string(path).unwrap();

    let monkeys = parse(content);

    println!("{:#?}", monkeys);
}
