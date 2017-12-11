use nom::{digit, alpha};
use std::str::{self, FromStr};

#[derive(PartialEq, Debug)]
pub struct Disc<'a> {
    pub name: &'a str,
    pub weight: u32,
    pub children: Option<Vec<&'a str>>
}

named!(number<&str, u32>,
       map_res!(
           digit,
//           map_res!(digit, str::from_utf8),
           FromStr::from_str
       )
);

named!(weight<&str, u32>,
       ws!(delimited!(
           tag!("("),
           number,
           tag!(")")
       ))
);

named!(children<&str, Vec<&str>>,
       ws!(do_parse!(
           tag!("->") >>
           list: separated_nonempty_list_complete!(tag!(","),
                                                   ws!(alpha)) >>
           (list)
       ))
);

named!(disc<&str, Disc>,
       ws!(do_parse!(
           name: alpha >>
           weight: weight >>
           children: opt!(complete!(children)) >>
               (Disc {
                   name: name,
                   weight: weight,
                   children: children
               })
       ))
);

named!(pub parse_discs<&str, Vec<Disc>>,
       many1!(disc)
);


#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;

    #[test]
    fn test_weight() {
        let input = "(15)";
        assert_eq!(weight(input), Done("", 15));
    }

    #[test]
    fn test_children() {
        let input = "-> foo, bar,baz";
        assert_eq!(children(input), Done("", vec!["foo", "bar", "baz"]));
        let input = "-> foo";
        assert_eq!(children(input), Done("", vec!["foo"]));
    }

    #[test]
    fn test_disc() {
        let input = "foo (16)";
        assert_eq!(disc(input), Done("", Disc {
            name: "foo",
            weight: 16,
            children: None
        }));

        let input = "bazz (1112) -> foo, bar";
        assert_eq!(disc(input), Done("", Disc {
            name: "bazz",
            weight: 1112,
            children: Some(vec!["foo", "bar"])
        }));
    }

    #[test]
    fn test_parse_discs() {
        let input = "foo (1) -> bar, baz\nabc (12)";
        assert_eq!(parse_discs(input), Done("", vec![
            Disc {
                name: "foo",
                weight: 1,
                children: Some(vec!["bar", "baz"])
            },
            Disc {
                name: "abc",
                weight: 12,
                children: None
            }
        ]));
    }
}
