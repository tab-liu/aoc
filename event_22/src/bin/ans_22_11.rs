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

    fn operation(&mut self) -> (i32, i32) {
        let item = self.items.pop_front().unwrap();
        let code = self.op.replace("old", &item.to_string());
        let new_item = eval(&code).unwrap().as_int().unwrap() as i32 / 3;
        self.inspect += 1;
        if new_item % self.div == 0 {
            (self.if_true, new_item)
        } else {
            (self.if_false, new_item)
        }
    }

    fn add_item(&mut self, item: i32) {
        self.items.push_back(item);
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

    fn round(&mut self) {
        for monkey_id in 0..self.group.len() {
            let times = self.group[monkey_id].items.len();
            for _ in 0..times {
                let (id, item) = self.group[monkey_id].operation();
                self.group[id as usize].add_item(item);
            }
        }
    }

    #[allow(dead_code)]
    fn show_items(&self) {
        for monkey in self.group.iter() {
            println!("id: {}, items: {:?}", monkey.id, monkey.items);
        }
        println!();
    }

    fn inspects(&self) -> Vec<i32> {
        self.group.iter().map(|m| m.inspect).collect()
    }

    fn level(&self) -> i32 {
        let mut ins = self.inspects();
        ins.sort_unstable_by(|a, b| b.cmp(a));
        ins[0] * ins[1]
    }
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
    path.push("src/bin/input_11.txt");

    let content = fs::read_to_string(path).unwrap();

    let mut monkeys = parse(content);

    // monkeys.show_items();
    // monkeys.round();
    // monkeys.show_items();

    for _ in 0..20 {
        monkeys.round();
    }
    monkeys.show_items();
    println!("{:?}", monkeys.inspects());
    println!("{:?}", monkeys.level());
}
