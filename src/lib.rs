pub mod parser;
pub mod env;
pub mod parser_options;
pub mod inline;

#[path = "plugins/marquee.rs"]
pub mod marquee;
#[path = "plugins/calendar.rs"]
pub mod calendar;
#[path = "plugins/ls.rs"]
pub mod ls;
#[path = "plugins/accordion.rs"]
pub mod accordion;
#[path = "plugins/include.rs"]
pub mod include;
#[path = "plugins/variable.rs"]
pub mod variable;

#[path = "plugins/comment/comment.rs"]
pub mod comment;
#[path = "plugins/comment/rtcomment.rs"]
pub mod rtcomment;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
use crate::parser_options::ParserOptions;
use crate::parser_options::ParseMode;

pub fn parse_wiki_input<T: AsRef<[u8]>>(
    input: T,
    wiki_slug: &str,
    page_slug: &str,
    options: &ParserOptions,
) -> String {
    let bytes = input.as_ref();
    let text = String::from_utf8_lossy(bytes);
    parser::parse(&text, bytes, wiki_slug, page_slug, options)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = parseWiki)]
pub fn parse_wiki(input: &str, wiki_slug: &str, page_slug: &str) -> String {
    let options = ParserOptions::default();
    parser::parse(input, input.as_bytes(), wiki_slug, page_slug, &options)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = parseWikiBinary)]
pub fn parse_wiki_binary(input: &[u8], wiki_slug: &str, page_slug: &str) -> String {
    let mut options = ParserOptions::default();
    options.mode = ParseMode::Binary;

    let text = String::from_utf8_lossy(input);
    parser::parse(&text, input, wiki_slug, page_slug, &options)
}