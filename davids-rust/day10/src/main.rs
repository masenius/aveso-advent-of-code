#[macro_use]
extern crate nom;

use std::str::{self, FromStr};
use std::collections::{HashMap, BTreeSet};
use nom::digit;

#[derive(Debug, PartialEq)]
enum Instruction {
    ValToBot {value: u32, bot: u32},
    BotGives {bot: u32, low_recipient: Recipient, high_recipient: Recipient}
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Recipient {
    Bot(u32),
    Output(u32)
}

named!(number<u32>,
       map_res!(
           map_res!(digit, str::from_utf8),
           FromStr::from_str
       )
);

named!(parse_val_to_bot<Instruction>,
       ws!(do_parse!(
           tag!("value") >>
           val: number >>
           tag!("goes to bot") >>
           bot: number >>
           (Instruction::ValToBot {value: val, bot: bot})
       ))
);

named!(output_or_bot<Recipient>,
       ws!(do_parse!(
           what: alt!(map_res!(tag!("bot"), str::from_utf8) |
                      map_res!(tag!("output"), str::from_utf8)) >>
               num: number >>
               (match what {
                   "bot" => Recipient::Bot(num),
                   "output" => Recipient::Output(num),
                   _ => unreachable!()
               })
       ))
);

named!(parse_bot_gives<Instruction>,
       ws!(do_parse!(
           tag!("bot") >>
           bot: number >>
           tag!("gives low to") >>
           low_recipient: output_or_bot >>
           tag!("and high to") >>
           high_recipient: output_or_bot >>
           (Instruction::BotGives {
               bot: bot,
               low_recipient: low_recipient,
               high_recipient: high_recipient
           })
       ))
);

named!(parse_instructions<Vec<Instruction> >,
       many1!(alt!(parse_bot_gives | parse_val_to_bot)));


type BotMap = HashMap<u32, BTreeSet<u32>>;
type OutputMap = HashMap<u32, u32>;

struct World {
    bots: BotMap,
    outputs: OutputMap
}

impl World {
    fn new() -> World {
        World {
            bots: BotMap::new(),
            outputs: OutputMap::new()
        }
    }

    fn bot_chips(&mut self, bot: u32) -> &mut BTreeSet<u32> {
        self.bots.entry(bot).or_insert(BTreeSet::new())
    }

    fn get_low_high(&self, bot: u32) -> Option<(u32, u32)> {
        if let Some(chips) = self.bots.get(&bot) {
            if chips.len() == 2 {
                let mut ci = chips.iter();
                let low = ci.nth(0).unwrap();
                let high = ci.nth(0).unwrap();
                return Some((*low, *high));
            }
        }
        None
    }

    fn transfer_chip_to_output(&mut self, output: u32, chip: u32) -> bool {
        if let Some(_) = self.outputs.insert(output, chip) {
            false
        }
        else {
            true
        }
    }

    fn get_output_value(&self, output: u32) -> Option<&u32> {
        self.outputs.get(&output)
    }
}

const LOW_CHIP: u32 = 17;
const HIGH_CHIP: u32 = 61;

fn is_responsible_bot(chips: &BTreeSet<u32>) -> bool {
    chips.contains(&LOW_CHIP) && chips.contains(&HIGH_CHIP)
}

fn transfer_chip_to_bot(recipient: u32, chip: u32, world: &mut World) -> (bool, &BTreeSet<u32>) {
    let nums = world.bot_chips(recipient);
    if nums.len() == 2 {
        (false, nums)
    }
    else {
        nums.insert(chip);
        (true, nums)
    }
}

fn make_transfer<F>(recipient: Recipient, value: u32, world: &mut World, on_bot_xfer: &F) -> bool
    where F: Fn(u32, &BTreeSet<u32>) {
    match recipient {
        Recipient::Bot(bn) => {
            let (is_new_xfer, chips) = transfer_chip_to_bot(bn, value, world);
            if is_new_xfer {
                on_bot_xfer(bn, chips);
                return true;
            }
        },
        Recipient::Output(on) => {
            if world.transfer_chip_to_output(on, value) {
                return true;
            }
        }
    }
    false
}

fn run_instructions<F>(instructions: &[Instruction], world: &mut World, on_bot_xfer: &F) -> bool
    where F: Fn(u32, &BTreeSet<u32>) {
    let mut transfer_made = false;
    for inst in instructions.iter() {
        if let Instruction::BotGives{bot, low_recipient, high_recipient} = *inst {
            if let Some((low, high)) = world.get_low_high(bot) {
                if make_transfer(low_recipient, low, world, on_bot_xfer) {
                    transfer_made = true;
                }
                if make_transfer(high_recipient, high, world, on_bot_xfer) {
                    transfer_made = true;
                }
            }

        }
    }
    transfer_made
}

fn on_bot_transfer(bot: u32, chips: &BTreeSet<u32>) {
    if is_responsible_bot(chips) {
        println!("Bot {} is responsible", bot);
    }
}

fn main() {
    let input = include_bytes!("input");
    let (_, instructions) = parse_instructions(input).unwrap();
    let mut world = World::new();
    for inst in instructions.iter() {
        if let Instruction::ValToBot{bot, value} = *inst {
            let nums = world.bot_chips(bot);
            nums.insert(value);
            assert!(nums.len() <= 2);
        }

    }

    while run_instructions(&instructions, &mut world, &on_bot_transfer) {}
    match (world.get_output_value(0), world.get_output_value(1), world.get_output_value(2)) {
        (Some(a), Some(b), Some(c)) => println!("Product of chips in output 0, 1 and 2 is {}", a * b * c),
        _ => panic!("One or more of outputs 0, 1, 2 have no chip")
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_val_to_bot, parse_bot_gives, parse_instructions};
    use super::Instruction::*;
    use super::Recipient::*;
    use nom::IResult::*;

    #[test]
    fn test_parse_val_to_bot() {
        let input = b"value 32 goes to bot 6\n";
        assert_eq!(parse_val_to_bot(input), Done(&b""[..], ValToBot {value: 32, bot: 6}));
    }

    #[test]
    fn test_parse_bot_gives() {
        let input = b"bot 1 gives low to output 1 and high to bot 0\n";
        assert_eq!(parse_bot_gives(input),
                   Done(&b""[..], BotGives {
                       bot: 1,
                       low_recipient: Output(1),
                       high_recipient: Bot(0)
                   }));
    }

    #[test]
    fn test_parse_instructions() {
        let input = b"value 32 goes to bot 6\nbot 1 gives low to output 1 and high to bot 0";
        assert_eq!(parse_instructions(input),
                   Done(&b""[..], vec![
                       ValToBot {value: 32, bot: 6},
                       BotGives {
                           bot: 1,
                           low_recipient: Output(1),
                           high_recipient: Bot(0)
                       }
                   ]));
    }
}
