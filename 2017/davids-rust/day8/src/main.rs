#[macro_use]
extern crate nom;

mod parser;

use parser::{Instruction, Action, Condition, Operator};

use std::collections::HashMap;
use std::cmp::{PartialOrd, PartialEq};

type Register = HashMap<String, i32>;

fn create_predicate(condition: &Condition) -> Box<Fn(&Register) -> bool> {
    use Operator::*;
    let target = String::from(condition.target);
    let number = condition.number;
    let pred: fn(&i32, &i32) -> bool = match condition.operator {
        Lt => PartialOrd::lt,
        Gt => PartialOrd::gt,
        Ge => PartialOrd::ge,
        Le => PartialOrd::le,
        Eq => PartialEq::eq,
        Ne => PartialEq::ne
    };
    Box::new(move |reg| {
        let val = reg.get(&target).unwrap_or(&0);
        pred(val, &number)
    })
}

fn create_action(action: &Action, amount: i32) -> Box<Fn(i32) -> i32> {
    match *action {
        Action::Inc => Box::new(move |n| n + amount),
        Action::Dec => Box::new(move |n| n - amount)
    }
}

fn process_instruction(instruction: &Instruction) -> Box<Fn(&mut Register)> {
    let action = create_action(&instruction.action, instruction.amount);
    let predicate = create_predicate(&instruction.condition);
    let target = String::from(instruction.target);
    Box::new(move |reg| {
        // This super awkward construction seems necessary if you
        // don't want to clone the key regardless of whether it already
        // exists in the map or not. The Entry API requires you to
        // pass ownership of the key no matter what.
        if predicate(reg) {
            if reg.contains_key(&target) {
                let val = reg.get_mut(&target).unwrap();
                *val = action(*val);
            }
            else {
                let val = action(0);
                reg.insert(target.clone(), val);
            }
        }
    })
}

fn main() {
    let input = include_str!("input");
    let instructions = parser::parse_instructions(input).to_result().unwrap();
    let mut register = Register::new();
    for instruction in instructions.iter() {
        process_instruction(&instruction)(&mut register);
    }
    let largest = register.values().max().unwrap();
    println!("Largest value in register: {}", largest);
}
