use evalexpr::*;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
#[derive(Debug, Default)]
struct Monkey {
    id: i64,
    items: VecDeque<i64>,
    op: String,
    div: i64,
    if_true: i64,
    if_false: i64,
    inspect: i64,
}

impl Monkey {
    fn new(
        id: i64,
        items: VecDeque<i64>,
        op: String,
        div: i64,
        if_true: i64,
        if_false: i64,
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

    fn operation(&mut self, worry_div: i64, mod_group: i64) -> (i64, i64) {
        let item = self.items.pop_front().unwrap();
        let code = self.op.replace("old", &item.to_string());
        let new_item = eval(&code).unwrap().as_int().unwrap() as i64 / worry_div;
        let new_item = if worry_div == 1 {
            new_item % mod_group
        } else {
            new_item
        };
        self.inspect += 1;
        if new_item % self.div == 0 {
            (self.if_true, new_item)
        } else {
            (self.if_false, new_item)
        }
    }

    fn add_item(&mut self, item: i64) {
        self.items.push_back(item);
    }
}

#[derive(Debug, Default)]
struct MonkeyGroup {
    mod_group: i64,
    group: Vec<Monkey>,
}

impl MonkeyGroup {
    fn new() -> Self {
        Default::default()
    }

    fn update_mode(&mut self) {
        self.mod_group = self
            .group
            .iter()
            .map(|x| x.div)
            .collect::<Vec<_>>()
            .iter()
            .product::<i64>();
    }

    fn round(&mut self, worry_div: i64) {
        for monkey_id in 0..self.group.len() {
            let times = self.group[monkey_id].items.len();
            for _ in 0..times {
                let (id, item) = self.group[monkey_id].operation(worry_div, self.mod_group);
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

    #[allow(dead_code)]
    fn inspects(&self) -> Vec<i64> {
        self.group.iter().map(|m| m.inspect).collect()
    }

    fn level(&self) -> i64 {
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
        .parse::<i64>()
        .unwrap();
    let items = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
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
        .parse::<i64>()
        .unwrap();
    let if_true = lines
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let if_false = lines
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<i64>()
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

    let mut monkeys = parse(content.clone());
    monkeys.update_mode();
    for _ in 0..20 {
        monkeys.round(3);
    }
    // monkeys.show_items();
    // println!("{:?}", monkeys.inspects());
    println!("{:?}", monkeys.level());

    // ------ part 2 --------
    let mut monkeys = parse(content);
    monkeys.update_mode();
    for _ in 0..10000 {
        monkeys.round(1);
    }
    // println!("{:?}", monkeys.inspects());
    println!("{:?}", monkeys.level());
}
