use std::fs;

use parser::parse::parse_html;

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
fn ytb() {
    // Open github source file

    const FILE: &str = include_str!(".././tests/sources/youtube_viewer.html");

    let parsed = parse_html(FILE);

    if let Err(ref why) = parsed {
        eprintln!("{why:?}")
    }

    assert!(parsed.is_ok());
}

#[test]
fn stackoverflow() {
    const FILE: &str = include_str!(".././tests/sources/stackoverflow.html");

    let parsed = parse_html(FILE);

    if let Err(ref why) = parsed {
        eprintln!("{why:?}")
    }

    assert!(parsed.is_ok());
}

#[test]
fn reflection() {
    const FILE: &str = include_str!(".././tests/sources/reflection.html");

    let parsed = parse_html(FILE);

    if let Err(ref why) = parsed {
        eprintln!("{why:?}")
    }

    assert!(parsed.is_ok());
}
