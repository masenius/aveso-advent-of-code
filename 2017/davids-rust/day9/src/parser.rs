use streamcontent::StreamContent;
use nom::anychar;

named!(pub group<StreamContent>,
       map!(delimited!(tag!("{"),
                       separated_list_complete!(
                           tag!(","), alt_complete!(garbage | group)),
                       tag!("}")
       ), StreamContent::Group)
);

fn vec_to_string(input: Vec<u8>) -> String {
    String::from_utf8_lossy(&input).into_owned()
}

named!(non_empty_garbage<StreamContent>,
       map!(delimited!(
           tag!("<"),
           map!(escaped_transform!(is_not!(">!"), '!', value!(&b""[..], anychar)), vec_to_string),
           tag!(">")
       ), StreamContent::Garbage)
);

// Due to some weird quirk with escape_transformed!, it doesn't seem to
// handle the case with empty garbage. Thus, a special case is necessary
named!(empty_garbage<StreamContent>,
       map!(
           value!(String::from(""), tag!("<>")),
           StreamContent::Garbage
       )
);

named!(garbage<StreamContent>,
       alt!(empty_garbage | non_empty_garbage)
);


#[cfg(test)]
mod test {
    use super::*;
    use super::StreamContent::*;
    use nom::IResult::*;

    #[test]
    fn test_garbage() {
        assert_eq!(garbage(b"<abcd>"), Done(&b""[..], Garbage(String::from("abcd"))));
        assert_eq!(garbage(b"<ab!>cd>"), Done(&b""[..], Garbage(String::from("abcd"))));
        assert_eq!(garbage(b"<ab!!>cd>"), Done(&b"cd>"[..], Garbage(String::from("ab"))));
        assert_eq!(garbage(b"<<<<>"), Done(&b""[..], Garbage(String::from("<<<"))));
        assert_eq!(garbage(b"<>"), Done(&b""[..], Garbage(String::from(""))));
        assert_eq!(garbage(br#"<{o"i!a,<{i<a>"#),
                   Done(&b""[..], Garbage(String::from(r#"{o"i,<{i<a"#))));
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
                           Garbage(String::from("},{<},{<},{<a"))
                       ])])));
        assert_eq!(group(b"{{},<sdsa{,}!>>}"),
                   Done(&b""[..], Group(vec![
                       Group(vec![]),
                       Garbage(String::from("sdsa{,}"))
                   ])));
    }
}
