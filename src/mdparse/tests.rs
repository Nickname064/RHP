use crate::mdparse::mdparse::{markdown_parse, MarkdownToken};

#[cfg(test)]

#[test]
fn title_quantify(){
    for i in 1 .. 6{
        let dbg_str = format!("{} TEXT", "#".repeat(i));
        let parsed = markdown_parse(&dbg_str);

        assert!(match parsed.first(){
            Some(MarkdownToken::TITLE(v)) => *v == i,
            _ => false
        })
    }
}

#[test]
fn title_identify(){
    let dbg_str = "SOME TEXT";
    assert!(match markdown_parse(dbg_str).first(){
        Some(MarkdownToken::TITLE(x)) => false,
        _ => true
    });
}