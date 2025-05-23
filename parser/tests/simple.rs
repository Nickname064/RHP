use parser::{errors::ParserError, parse::parse_html};

#[test]
fn test_simple_text() {
    assert!(parse_html("Hello World!").is_ok())
}

#[test]
fn simple_tag() {
    assert!(parse_html("<div>Hello</div>").is_ok())
}

#[test]
fn simple_err() {
    assert!(parse_html("<div>Hello</div").is_err())
}

#[test]
fn contained() {
    assert!(parse_html("<div><button>Click to proceed</button></div>").is_ok())
}

#[test]
fn script() {
    assert!(parse_html("<script>(() => { console.log('Hello World !') })();</script>").is_ok())
}

#[test]
fn ytb() -> Result<(), ParserError> {
    parse_html(include_str!(".././tests/sources/youtube_viewer.html")).map(|_| ())
}

#[test]
fn stackoverflow() -> Result<(), ParserError> {
    parse_html(include_str!(".././tests/sources/stackoverflow.html")).map(|_| ())
}

#[test]
fn reflection() -> Result<(), ParserError> {
    parse_html(include_str!(".././tests/sources/reflection.html")).map(|_| ())
}
