use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space0, space1},
    multi::{many0, separated_list},
    sequence::{pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Header {
    tikz_library: Vec<String>,
    package: Vec<String>,
}

impl Header {
    fn new() -> Header {
        Header {
            tikz_library: vec![],
            package: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
enum Item {
    TikzLibrary(String),
    Package(String),
}

fn prefix(input: &str) -> IResult<&str, (&str, &str)> {
    pair(tag("%%"), space1)(input)
}

fn colon(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((space0, tag(":"), space0))(input)
}

fn separator(input: &str) -> IResult<&str, (&str, &str)> {
    pair(tag(","), space0)(input)
}

fn tikz_library(input: &str) -> IResult<&str, Vec<Item>> {
    let (input, _) = pair(tag("tikzlibrary"), colon)(input)?;
    let (input, libs) = separated_list(separator, alpha1)(input)?;
    Ok((
        input,
        libs.iter()
            .map(|s| Item::TikzLibrary(s.to_string()))
            .collect(),
    ))
}

fn package(input: &str) -> IResult<&str, Vec<Item>> {
    let (input, _) = pair(tag("package"), colon)(input)?;
    let (input, packages) = separated_list(separator, alpha1)(input)?;
    Ok((
        input,
        packages
            .iter()
            .map(|s| Item::Package(s.to_string()))
            .collect(),
    ))
}

fn header_line(input: &str) -> IResult<&str, Vec<Vec<Item>>> {
    let component = alt((tikz_library, package));
    let (input, _) = prefix(input)?;
    let (input, items) = separated_list(space1, component)(input)?;
    let (input, _) = pair(space0, tag("\n"))(input)?;
    Ok((input, items))
}

pub fn header(input: &str) -> IResult<&str, Header> {
    let (input, items) = many0(header_line)(input)?;
    let mut header = Header::new();
    for item in items.into_iter().flatten().flatten() {
        match item {
            Item::TikzLibrary(name) => header.tikz_library.push(name),
            Item::Package(name) => header.package.push(name),
        }
    }
    Ok((input, header))
}

#[test]
fn test_tikz_library() {
    assert_eq!(
        tikz_library("tikzlibrary: calc, arrows"),
        Ok((
            "",
            ["calc", "arrows"]
                .iter()
                .map(|s| Item::TikzLibrary(s.to_string()))
                .collect()
        ))
    );
    assert_eq!(
        tikz_library("tikzlibrary:calc,arrows"),
        Ok((
            "",
            ["calc", "arrows"]
                .iter()
                .map(|s| Item::TikzLibrary(s.to_string()))
                .collect()
        ))
    );
}

#[test]
fn test_package() {
    assert_eq!(
        package("package: tikzcd, ctex"),
        Ok((
            "",
            ["tikzcd", "ctex"]
                .iter()
                .map(|s| Item::Package(s.to_string()))
                .collect()
        ))
    );
    assert_eq!(
        package("package:ctex"),
        Ok(("", vec![Item::Package("ctex".to_owned())]))
    );
}

#[test]
fn test_header() {
    assert_eq!(
        header(
            r#"%% tikzlibrary: calc, decorate
%% package:ctex  tikzlibrary: arrows
"#
        ),
        Ok((
            "",
            Header {
                tikz_library: vec![
                    "calc".to_owned(),
                    "decorate".to_owned(),
                    "arrows".to_owned()
                ],
                package: vec!["ctex".to_owned()]
            }
        ))
    );
}
