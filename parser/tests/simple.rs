use parser::parse::parse_html;

#[test]
fn test_simple_text() {
    assert!(parse_html("Hello World!").is_ok())
}

#[test]
fn test_simple_tag() {
    assert!(parse_html("<div>Hello</div>").is_ok())
}

#[test]
fn test_simple_err() {
    assert!(parse_html("<div>Hello</div").is_err())
}
