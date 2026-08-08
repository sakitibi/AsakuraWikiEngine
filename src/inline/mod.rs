pub mod binary;
pub mod text;

use crate::parser_options::{ParseMode, ParserOptions};

/// インライン要素の解析エントリポイント
pub fn parse_inline_elements(text: &str, bytes: &[u8], options: &ParserOptions) -> String {
    match options.mode {
        ParseMode::Binary => binary::parse_binary_inline(bytes, options),
        ParseMode::Text => text::parse_text_inline(text, options),
    }
}

pub fn trim_bytes(slice: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = slice.len();

    while start < end && slice[start].is_ascii_whitespace() {
        start += 1;
    }
    while end > start && slice[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    &slice[start..end]
}