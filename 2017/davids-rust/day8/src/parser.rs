use nom::{digit, alpha};
use std::str::FromStr;


#[derive(Debug, PartialEq)]
pub enum Operator {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le
}

impl Operator {
    fn from_str(input: &str) -> Result<Self, ()> {
        use self::Operator::*;
        match input {
            "==" => Ok(Eq),
            "!=" => Ok(Ne),
            ">" => Ok(Gt),
            "<" => Ok(Lt),
            ">=" => Ok(Ge),
            "<=" => Ok(Le),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Inc,
    Dec
}

impl Action {
    fn from_str(input: &str) -> Result<Self, ()> {
        use self::Action::*;
        match input {
            "inc" => Ok(Inc),
            "dec" => Ok(Dec),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction<'a> {
    pub target: &'a str,
    pub action: Action,
    pub amount: i32,
    pub condition: Condition<'a>
}

#[derive(Debug, PartialEq)]
pub struct Condition<'a> {
    pub target: &'a str,
    pub operator: Operator,
    pub number: i32
}

named!(signed_number<&str, i32>,
       map_res!(
           recognize!(pair!(opt!(tag!("-")), digit)),
           FromStr::from_str
       )
);

named!(operator<&str, Operator>,
       map_res!(alt_complete!(tag!("==") |
                              tag!("!=") |
                              tag!(">=") |
                              tag!("<=") |
                              tag!(">") |
                              tag!("<")),
                Operator::from_str));

named!(action<&str, Action>,
       map_res!(alt!(tag!("inc") |
                     tag!("dec")),
                Action::from_str));

named!(condition<&str, Condition>,
       ws!(do_parse!(
           tag!("if") >>
               target: alpha >>
               operator: operator >>
               number: signed_number >>
               (Condition {
                   target: target,
                   operator: operator,
                   number: number
               })
       ))
);

named!(instruction<&str, Instruction>,
       ws!(do_parse!(
           target: alpha >>
               action: action >>
               amount: signed_number >>
               condition: condition >>
               (Instruction {
                   target: target,
                   action: action,
                   amount: amount,
                   condition: condition
               })
       ))
);

named!(pub parse_instructions<&str, Vec<Instruction>>,
       many1!(instruction)
);


#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_operator() {
        assert_eq!(operator("!="), Done("", Operator::Ne));
        assert_eq!(operator(">="), Done("", Operator::Ge));
        assert_eq!(operator("<"), Done("", Operator::Lt));
    }

    #[test]
    fn test_action() {
        assert_eq!(action("dec"), Done("", Action::Dec));
        assert_eq!(action("inc"), Done("", Action::Inc));
    }

    #[test]
    fn test_signed_number() {
        assert_eq!(signed_number("-52"), Done("", -52));
        assert_eq!(signed_number("52"), Done("", 52));
    }

    #[test]
    fn test_condition() {
        assert_eq!(condition("if foo > 2"), Done("", Condition {
            target: "foo",
            operator: Operator::Gt,
            number: 2
        }));
        assert_eq!(condition("if xysda == -79"), Done("", Condition {
            target: "xysda",
            operator: Operator::Eq,
            number: -79
        }));
    }

    #[test]
    fn test_instruction() {
        assert_eq!(instruction("bar dec 552 if foo > 2"), Done("", Instruction {
            target: "bar",
            action: Action::Dec,
            amount: 552,
            condition: Condition {
                target: "foo",
                operator: Operator::Gt,
                number: 2
            }})
        );
        assert_eq!(instruction("h dec 192 if u <= 5"), Done("", Instruction {
            target: "h",
            action: Action::Dec,
            amount: 192,
            condition: Condition {
                target: "u",
                operator: Operator::Le,
                number: 5
            }})
        );
    }

    #[test]
    fn test_parse_instructions() {
        assert_eq!(parse_instructions("bar dec 552 if foo > 2\nxux inc 2 if abc != -8"),
                   Done("", vec![Instruction {
                       target: "bar",
                       action: Action::Dec,
                       amount: 552,
                       condition: Condition {
                           target: "foo",
                           operator: Operator::Gt,
                           number: 2
                       }}, Instruction {
                       target: "xux",
                       action: Action::Inc,
                       amount: 2,
                       condition: Condition {
                           target: "abc",
                           operator: Operator::Ne,
                           number: -8
                       }}])
        );

    }
}
