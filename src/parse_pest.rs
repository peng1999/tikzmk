use pest::{error::Error, Parser};
use serde::Serialize;

#[derive(Parser)]
#[grammar = "header.pest"]
struct HeaderParser;

#[derive(Debug, PartialEq, Serialize)]
pub struct Header {
    tikz_library: Vec<String>,
    package: Vec<String>,
}

impl Header {
    pub fn new() -> Header {
        Header {
            tikz_library: vec![],
            package: vec![],
        }
    }
}

pub fn header(input: &str) -> Result<Header, Error<Rule>> {
    let lines = HeaderParser::parse(Rule::header, input)?.next().unwrap();
    println!("{:#?}", lines);
    let mut header = Header::new();

    for line in lines.into_inner() {
        // line: header_lines
        for decl in line.into_inner() {
            // decl: tikz_library | package
            let kind = decl.as_rule();
            let names = decl.into_inner().map(|p| p.as_str().to_string());
            match kind {
                Rule::tikz_library => {
                    header.tikz_library.extend(names);
                }
                Rule::package => {
                    header.package.extend(names);
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(header)
}

#[test]
fn test_header_parser() {
    let txt = "%%\n%%  tikzlibrary:calc,arrow\n";
    assert!(HeaderParser::parse(Rule::header, txt).is_ok());
    assert_eq!(
        header(txt),
        Ok(Header {
            tikz_library: vec!["calc".to_string(), "arrow".to_string()],
            package: vec![]
        })
    );
}
