use aoc_runner_derive::aoc;

use std::fmt;

#[derive(Debug)]
enum Operator {
    MULTIPLY,
    ADD,
}

#[derive(Debug)]
enum Target {
    SELF,
    STATIC(u64),
}

struct Monkey {
    inventory: Vec<u64>,
    operation_target: Target,
    operation_operator: Operator,
    test_value: u64,
    test_true: u64,
    test_false: u64,
    items_inspected: u64,
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
    fn inspect_item(&self, old: u64) -> u64 {
        match (&self.operation_target, &self.operation_operator) {
            (Target::SELF, Operator::ADD) => old + old,
            (Target::SELF, Operator::MULTIPLY) => old * old,
            (Target::STATIC(value), Operator::ADD) => old + value,
            (Target::STATIC(value), Operator::MULTIPLY) => old * value,
        }
    }
    fn throw_item_to(&self, item_worry_level: u64) -> u64 {
        if item_worry_level % self.test_value == 0 {
            return self.test_true;
        }
        return self.test_false;
    }
}

fn get_last_int(line: &str) -> u64 {
    line.split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

fn get_monkeys_from_input(content: &str) -> Vec<Monkey> {
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
        let items: Vec<u64> = starting
            .split(", ")
            .map(|c| c.parse::<u64>().unwrap())
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
            value => Target::STATIC(value.parse::<u64>().unwrap()),
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
    monkeys
}

fn monkey_turn(monkey: &mut Monkey, combined_modulo: u64) -> Vec<(u64, u64)> {
    // Keep track of monkeyids/items thrown by this monkey
    // (cannot modify `monkeys` here due to borrowing restrictions)
    let mut items_thrown: Vec<(u64, u64)> = Vec::new();
    for &item_wl_old in monkey.inventory.iter() {
        let mut item_wl_new = monkey.inspect_item(item_wl_old);
        if combined_modulo == 0 {
            // part 1
            item_wl_new = (item_wl_new as f32 / 3_f32).floor() as u64;
        } else {
            // part 2: apply product of all test values as modulo, because that won't change the 'throw item to' division
            item_wl_new = item_wl_new % combined_modulo;
        }
        let monkey_id = monkey.throw_item_to(item_wl_new);
        items_thrown.push((monkey_id, item_wl_new));
    }
    monkey.items_inspected += items_thrown.len() as u64;
    // Clear this monkey's inventory
    monkey.inventory = Vec::new();
    items_thrown
}

fn monkey_business(monkeys: &mut [Monkey], combined_modulo: u64) {
    for this_monkey in 0..monkeys.len() {
        let mut monkey = &mut monkeys[this_monkey];
        let items_thrown = monkey_turn(&mut monkey, combined_modulo);
        for (other_monkey, item_wl) in items_thrown {
            monkeys[other_monkey as usize].inventory.push(item_wl);
        }
    }
}

#[aoc(day11, part1)]
fn part1(content: &str) -> u64 {
    let mut monkeys = get_monkeys_from_input(content);
    let mut round = 0;
    loop {
        round += 1;
        monkey_business(&mut monkeys, 0);
        if round == 20 {
            break;
        }
    }

    let mut item_inspections: Vec<u64> = monkeys.iter().map(|m| m.items_inspected).collect();
    item_inspections.sort();
    let highest = &item_inspections[item_inspections.len() - 2..];
    highest[0] as u64 * highest[1] as u64
    // 58322
}

#[aoc(day11, part2)]
fn part2(content: &str) -> u64 {
    let mut monkeys = get_monkeys_from_input(content);
    let combined_modulo: u64 = monkeys.iter().map(|m| m.test_value).product();
    let mut round = 0;
    loop {
        round += 1;
        monkey_business(&mut monkeys, combined_modulo);
        if round == 10000 {
            break;
        }
    }

    let mut item_inspections: Vec<u64> = monkeys.iter().map(|m| m.items_inspected).collect();
    item_inspections.sort();
    let highest = &item_inspections[item_inspections.len() - 2..];
    highest[0] as u64 * highest[1] as u64
    // 13937702909
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 10605u64);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 2713310158u64);
    }
}
