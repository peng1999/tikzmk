use log::{error, warn};
use regex::Regex;
use serde::Serialize;
use tinytemplate::TinyTemplate;

use crate::parse::{self, Header};

#[derive(Serialize)]
struct Context {
    content: String,
    header: Header,
}

const TEMPLATE: &str = r"\documentclass[tikz]\{standalone}{{for pkg in header.package}}
\usepackage\{{pkg}}{{endfor}}{{for lib in header.tikz_library}}
\usetikzlibrary\{{lib}}{{endfor}}
\begin\{document}
{content | unescaped}
\end\{document}
";

fn strip_bom(text: &str) -> &str {
    if text.starts_with("\u{FEFF}") {
        &text[3..]
    } else {
        text
    }
}

pub fn render(file_text: &str) -> String {
    let file_text = strip_bom(file_text);
    let pat = Regex::new(r"\A(\s*\n)*(%%( .*)?\n)+").unwrap();
    let maybe_mat = pat.find(file_text);
    let header = maybe_mat // Option<Match>
        .and_then(|mat| {
            parse::header(mat.as_str())
                .map_err(|err| {
                    error!("error: {:?}", err);
                })
                .ok()
        }) // Option<(&str, Header)>
        .map(|(remain, header)| {
            if remain.len() != 0 {
                warn!("Header parse may be incomplete.")
            }
            header
        }) // Option<Header>
        .unwrap_or(Header::new());

    let content = maybe_mat
        .map(|mat| &file_text[mat.end()..])
        .unwrap_or(file_text)
        .to_string();

    let mut tt = TinyTemplate::new();
    tt.add_template("latex", TEMPLATE).unwrap();

    tt.render("latex", &Context { content, header }).unwrap()
}

#[test]
fn test_render() {
    let origin = r"%%
%% tikzlibrary:l package:p,q
\abc{de}";
    let rendered = r"\documentclass[tikz]{standalone}
\usepackage{p}
\usepackage{q}
\usetikzlibrary{l}
\begin{document}
\abc{de}
\end{document}
";
    assert_eq!(render(origin), rendered);
}

#[test]
fn test_render_bom() {
    let origin = "\u{FEFF}%% tikzlibrary:calc
\\abc{de}";
    let rendered = r"\documentclass[tikz]{standalone}
\usetikzlibrary{calc}
\begin{document}
\abc{de}
\end{document}
";
    assert_eq!(render(origin), rendered);
}

#[test]
fn test_render_arrow() {
    let origin = r"\abc[<->]";
    let rendered = r"\documentclass[tikz]{standalone}
\begin{document}
\abc[<->]
\end{document}
";
    assert_eq!(render(origin), rendered);
}
