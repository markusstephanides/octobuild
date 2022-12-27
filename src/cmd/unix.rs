use std::io::{Error, ErrorKind};

// Parsing command line arguments from singe line.
pub fn parse(cmd: &str) -> Result<Vec<String>, Error> {
    shlex::split(cmd).ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to parse"))
}

#[test]
fn test_parse_1() {
    assert_eq!(parse(r#""abc" d e"#).unwrap(), ["abc", "d", "e"]);
}

#[test]
fn test_parse_2() {
    assert_eq!(parse(r#" "abc" d e "#).unwrap(), ["abc", "d", "e"]);
}

#[test]
fn test_parse_3() {
    assert_eq!(
        parse(r#""" "abc" d e """#).unwrap(),
        ["", "abc", "d", "e", ""]
    );
}

#[test]
fn test_parse_4() {
    assert_eq!(
        parse(r#"a\\b d"e f"g h"#).unwrap(),
        [r#"a\b"#, "de fg", "h"]
    );
}

#[test]
fn test_parse_5() {
    assert_eq!(parse(r#"a\\\"b c d"#).unwrap(), [r#"a\"b"#, "c", "d"]);
}

#[test]
fn test_parse_6() {
    assert_eq!(parse(r#"a\\\\"b c" d e"#).unwrap(), [r#"a\\b c"#, "d", "e"]);
}

#[test]
fn test_parse_7() {
    assert_eq!(
        parse(r#"C:\\Windows\\System32 d e"#).unwrap(),
        [r#"C:\Windows\System32"#, "d", "e"]
    );
}

#[test]
fn test_parse_8() {
    assert_eq!(
        parse(r#"/TEST"C:\Windows\System32" d e"#).unwrap(),
        [r#"/TESTC:\Windows\System32"#, "d", "e"]
    );
}

#[test]
fn test_parse_9() {
    assert_eq!(
        parse(r#"begin ' some text " foo\ bar\' end"#).unwrap(),
        ["begin", r#" some text " foo\ bar\"#, "end"]
    );
}

#[test]
fn test_parse_10() {
    assert_eq!(
        parse(r#"begin some\ text end"#).unwrap(),
        ["begin", "some text", "end"]
    );
}
