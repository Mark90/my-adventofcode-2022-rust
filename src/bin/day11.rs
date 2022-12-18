use aoc2022::utils;
use std::fmt;
use std::fs::read_to_string;

const DAY: &str = "day11";

#[derive(Debug)]
enum Operator {
    MULTIPLY,
    ADD,
}

#[derive(Debug)]
enum Target {
    SELF,
    STATIC(i32),
}

struct Monkey {
    inventory: Vec<i32>,
    operation_target: Target,
    operation_operator: Operator,
    test_value: i32,
    test_true: i32,
    test_false: i32,
    items_inspected: i32,
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "inspected {} inventory {:?}",
            self.items_inspected, self.inventory
        )
    }
}

impl Monkey {
    fn inspect_item(&self, old: i32) -> i32 {
        match (&self.operation_target, &self.operation_operator) {
            (Target::SELF, Operator::ADD) => old + old,
            (Target::SELF, Operator::MULTIPLY) => old * old,
            (Target::STATIC(value), Operator::ADD) => old + value,
            (Target::STATIC(value), Operator::MULTIPLY) => old * value,
        }
    }
    fn throw_item_to(&self, item_worry_level: i32) -> i32 {
        if item_worry_level % self.test_value == 0 {
            return self.test_true;
        }
        return self.test_false;
    }
}

fn get_last_int(line: &str) -> i32 {
    line.split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap()
}

fn part1(content: &str) -> i32 {
    let mut monkeys = Vec::new();

    for lines in content.split("\n\n") {
        let mut split_lines = lines.lines().skip(1);
        let starting = split_lines
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap();
        let items: Vec<i32> = starting
            .split(", ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect();
        let line_operation: Vec<&str> = split_lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .collect();

        let operator = line_operation[line_operation.len() - 2];
        let target = line_operation[line_operation.len() - 1];
        let divide_by = get_last_int(split_lines.next().unwrap());
        let test_true = get_last_int(split_lines.next().unwrap());
        let test_false = get_last_int(split_lines.next().unwrap());

        let operation_operator = match operator {
            "*" => Operator::MULTIPLY,
            "+" => Operator::ADD,
            _ => unreachable!("imp"),
        };
        let operation_target = match target {
            "old" => Target::SELF,
            value => Target::STATIC(value.parse::<i32>().unwrap()),
        };
        monkeys.push(Monkey {
            inventory: items,
            operation_operator: operation_operator,
            operation_target: operation_target,
            test_value: divide_by,
            test_true: test_true,
            test_false: test_false,
            items_inspected: 0,
        })
    }

    println!("Before start");
    log_inventories(&monkeys);

    let mut round = 0;
    loop {
        round += 1;
        monkey_business(&mut monkeys);
        println!("After round {}", round);
        log_inventories(&monkeys);

        if round == 20 {
            break;
        }
    }

    let mut item_inspections: Vec<i32> = monkeys.iter().map(|m| m.items_inspected).collect();
    item_inspections.sort();
    let highest = &item_inspections[item_inspections.len() - 2..];
    highest[0] * highest[1]
}

fn log_inventories(monkeys: &Vec<Monkey>) {
    for (i, m) in monkeys.iter().enumerate() {
        println!("monkey {}: {}", i, m);
    }
}

fn monkey_business(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        // println!("Monkey {}'s turn", i);
        let mut monkey = &mut monkeys[i];
        let items_thrown = monkey_turn(&mut monkey);
        for (monkey_id, item_wl) in items_thrown {
            // Append thrown items to the end of the recipient monkey's inventory
            monkeys[monkey_id as usize].inventory.push(item_wl);
        }
    }
}

fn monkey_turn(monkey: &mut Monkey) -> Vec<(i32, i32)> {
    // Keep track of monkeyids/items thrown by this monkey
    // (cannot modify `monkeys` here due to borrowing restrictions)
    let mut items_thrown: Vec<(i32, i32)> = Vec::new();
    for &item_wl_old in monkey.inventory.iter() {
        let item_wl_new = ((monkey.inspect_item(item_wl_old) as f32) / 3_f32).floor() as i32;
        // println!("{} worry level is now {}", item_wl_old, item_wl_new);
        let monkey_id = monkey.throw_item_to(item_wl_new);
        items_thrown.push((monkey_id, item_wl_new));
    }
    monkey.items_inspected += items_thrown.len() as i32;
    // Clear this monkey's inventory
    monkey.inventory = Vec::new();
    items_thrown
}

fn part2(content: &str) -> i32 {
    0
}

fn main() {
    let content = read_to_string(utils::get_path(DAY, false)).expect("File not found");
    println!("part1 {}", part1(&content)); // 58322
    println!("part2 {}", part2(&content));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
        assert_eq!(part1(&content), 10605);
    }

    // #[test]
    // fn test_part_2() {
    //     let content = read_to_string(utils::get_path(DAY, true)).expect("File not found");
    //     assert_eq!(part2(&content), -1);
    // }
}
