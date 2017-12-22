use std::str;

use streamcontent::StreamContent;
use nom::anychar;

named!(pub group<StreamContent>,
       map!(delimited!(tag!("{"),
                       separated_list_complete!(
                           tag!(","), alt_complete!(garbage | group)),
                       tag!("}")
       ), StreamContent::Group)
);

named!(garbage<StreamContent>,
       map!(delimited!(
           tag!("<"),
           map_res!(escaped!(is_not!(">!"), '!', anychar), str::from_utf8),
           tag!(">")
       ), StreamContent::Garbage)
);


#[cfg(test)]
mod test {
    use super::*;
    use super::StreamContent::*;
    use nom::IResult::*;

    #[test]
    fn test_garbage() {
        assert_eq!(garbage(b"<abcd>"), Done(&b""[..], Garbage("abcd")));
        assert_eq!(garbage(b"<ab!>cd>"), Done(&b""[..], Garbage("ab!>cd")));
        assert_eq!(garbage(b"<ab!!>cd>"), Done(&b"cd>"[..], Garbage("ab!!")));
        assert_eq!(garbage(b"<<<<>"), Done(&b""[..], Garbage("<<<")));
        assert_eq!(garbage(b"<>"), Done(&b""[..], Garbage("")));
        assert_eq!(garbage(br#"<{o"i!a,<{i<a>"#),
                   Done(&b""[..], Garbage(r#"{o"i!a,<{i<a"#)));
    }

    #[test]
    fn test_group() {
        assert_eq!(group(b"{}"), Done(&b""[..], Group(vec![])));
        assert_eq!(group(b"{{{}}}"), Done(&b""[..],
                                          Group(vec![
                                              Group(vec![
                                                  Group(vec![])
                                              ])])));
        assert_eq!(group(b"{{{},{},{{}}}}"), Done(&b""[..],
                                                  Group(vec![
                                                      Group(vec![
                                                          Group(vec![]),
                                                          Group(vec![]),
                                                          Group(vec![
                                                              Group(vec![])
                                                          ])])])));
        assert_eq!(group(b"{{<!>},{<!>},{<!>},{<a>}}"),
                   Done(&b""[..], Group(vec![
                       Group(vec![
                           Garbage("!>},{<!>},{<!>},{<a")
                       ])])));
        assert_eq!(group(b"{{},<sdsa{,}!>>}"),
                   Done(&b""[..], Group(vec![
                       Group(vec![]),
                       Garbage("sdsa{,}!>")
                   ])));
    }
}
