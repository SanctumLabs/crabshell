use crabshell::utils::string_utils::tokenize;

#[test]
fn tokenize_basic_cases() {
    assert_eq!(tokenize("foo bar"), vec!["foo", "bar"]);
    assert_eq!(tokenize("a  b\tc"), vec!["a", "b", "c"]);
}

#[test]
fn tokenize_with_quotes_and_escapes() {
    assert_eq!(tokenize("'a b' c"), vec!["a b", "c"]);
    assert_eq!(tokenize(r#""a b" c"#), vec!["a b", "c"]);
    assert_eq!(tokenize(r#"foo\ bar baz"#), vec!["foo bar", "baz"]);
    assert_eq!(tokenize(r#"foo"bar"baz"#), vec!["foobarbaz"]);
    assert_eq!(tokenize(r#"foo 'bar' baz"#), vec!["foo", "bar", "baz"]);
}
