use crate::parser_options::ParserOptions;

pub fn parse_text_inline(text: &str, options: &ParserOptions) -> String {
    let mut chars: Vec<char> = text.chars().collect();

    // &br; インラインの判定
    if options.br_inline {
        let text_str: String = chars.iter().collect();
        let replaced = text_str.replace("&br;", "<br>");
        chars = replaced.chars().collect();
    }

    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        // 太字 (''bold'') の判定
        if i + 1 < chars.len() && chars[i] == '\'' && chars[i + 1] == '\'' {
            let mut found_close = false;
            let mut bold_content = String::new();
            i += 2;

            while i < chars.len() {
                if i + 1 < chars.len() && chars[i] == '\'' && chars[i + 1] == '\'' {
                    i += 2;
                    found_close = true;
                    break;
                }
                bold_content.push(chars[i]);
                i += 1;
            }

            if found_close {
                result.push_str("<strong>");
                result.push_str(&bold_content);
                result.push_str("</strong>");
            } else {
                result.push_str("''");
                result.push_str(&bold_content);
            }
            continue;
        }

        // Wiki リンク ([[title>target]] または [[page]]) の判定
        if i + 1 < chars.len() && chars[i] == '[' && chars[i + 1] == '[' {
            let mut found_close = false;
            let mut link_content = String::new();
            i += 2;

            while i < chars.len() {
                if i + 1 < chars.len() && chars[i] == ']' && chars[i + 1] == ']' {
                    i += 2;
                    found_close = true;
                    break;
                }
                link_content.push(chars[i]);
                i += 1;
            }

            if found_close {
                let split_pos = link_content.find(|c| c == '>' || c == ':');

                if let Some(pos) = split_pos {
                    let title = link_content[..pos].trim();
                    let target = link_content[pos + 1..].trim();

                    let is_external = target.starts_with("http://") || target.starts_with("https://");
                    let href_val = if is_external { target } else { "#" };

                    result.push_str(&format!(
                        "<a href=\"{}\" data-wiki-link=\"{}\" class=\"wiki-link\">{}</a>",
                        href_val, target, title
                    ));
                } else {
                    let page_name = link_content.trim();
                    result.push_str(&format!(
                        "<a href=\"#\" data-wiki-link=\"{}\" class=\"wiki-link\">{}</a>",
                        page_name, page_name
                    ));
                }
            } else {
                result.push_str("[[");
                result.push_str(&link_content);
            }
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}