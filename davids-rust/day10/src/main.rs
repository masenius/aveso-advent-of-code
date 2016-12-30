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


const LOW_CHIP: u32 = 17;
const HIGH_CHIP: u32 = 61;

fn is_responsible_bot(chips: &BTreeSet<u32>) -> bool {
    chips.contains(&LOW_CHIP) && chips.contains(&HIGH_CHIP)
}

fn transfer_chip(recipient: u32, chip: u32, bots: &mut HashMap<u32, BTreeSet<u32>>) -> &BTreeSet<u32> {
    let nums = bots.entry(recipient).or_insert(BTreeSet::new());
    nums.insert(chip);
    nums
}

fn run_instructions(instructions: &[Instruction], bots: &mut HashMap<u32, BTreeSet<u32>>) -> Option<u32> {
    for inst in instructions.iter() {
        if let Instruction::BotGives{bot, low_recipient, high_recipient} = *inst {
            if let Some(nums) = bots.get(&bot).cloned() {
                if nums.len() == 2 {
                    let mut ni = nums.iter();
                    let low = ni.nth(0).unwrap();
                    let high = ni.nth(0).unwrap();
                    if let Recipient::Bot(bn) = low_recipient {
                        let chips = transfer_chip(bn, *low, bots);
                        if is_responsible_bot(chips) {
                            return Some(bn);
                        }
                    }
                    if let Recipient::Bot(bn) = high_recipient {
                        let chips = transfer_chip(bn, *high, bots);
                        if is_responsible_bot(chips) {
                            return Some(bn);
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let input = include_bytes!("input");
    let (_, instructions) = parse_instructions(input).unwrap();
    let mut bots = HashMap::new();
    for inst in instructions.iter() {
        if let Instruction::ValToBot{bot, value} = *inst {
            let nums = bots.entry(bot).or_insert(BTreeSet::new());
            nums.insert(value);
            assert!(nums.len() <= 2);
        }

    }

    loop {
        if let Some(bot) = run_instructions(&instructions, &mut bots) {
            println!("Bot {} is responsible", bot);
            break;
        }
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
