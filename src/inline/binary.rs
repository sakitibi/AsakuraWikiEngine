use crate::parser_options::ParserOptions;
use super::trim_bytes;

pub fn parse_binary_inline(bytes: &[u8], options: &ParserOptions) -> String {
    let binary_data = bytes;
    let mut result_bytes = Vec::new();
    let mut i = 0;

    while i < binary_data.len() {
        let byte = binary_data[i];

        // 1. 0x01 を <br> タグに置換 (options.br_inline が有効な場合)
        if options.br_inline && byte == b'\x01' {
            result_bytes.extend_from_slice(b"<br>");
            i += 1;
            continue;
        }

        // 2. 太字 (0x02...0x02) の判定
        if byte == b'\x02' {
            let start_idx = i;
            let mut found_close = false;
            let mut bold_content = Vec::new();
            i += 1;

            while i < binary_data.len() {
                if binary_data[i] == b'\x02' {
                    i += 1;
                    found_close = true;
                    break;
                }
                bold_content.push(binary_data[i]);
                i += 1;
            }

            if found_close {
                result_bytes.extend_from_slice(b"<strong>");
                result_bytes.extend_from_slice(&bold_content);
                result_bytes.extend_from_slice(b"</strong>");
            } else {
                result_bytes.extend_from_slice(&binary_data[start_idx..i]);
            }
            continue;
        }

        // 3. リンク (0x03 テキスト 0x04 URL 0x03) の判定
        if byte == b'\x03' {
            let start_idx = i;
            i += 1;

            let mut first_part = Vec::new();
            let mut second_part = Vec::new();
            let mut found_separator = false;
            let mut found_close = false;

            while i < binary_data.len() {
                let current_byte = binary_data[i];

                if current_byte == b'\x03' {
                    i += 1;
                    found_close = true;
                    break;
                } else if current_byte == b'\x04' && !found_separator {
                    found_separator = true;
                    i += 1;
                    continue;
                }

                if found_separator {
                    second_part.push(current_byte);
                } else {
                    first_part.push(current_byte);
                }
                i += 1;
            }

            if found_close {
                if found_separator {
                    let title = trim_bytes(&first_part);
                    let target = trim_bytes(&second_part);

                    let is_external = target.starts_with(b"http://") || target.starts_with(b"https://");
                    let href_val = if is_external { target } else { b"#" as &[u8] };

                    result_bytes.extend_from_slice(b"<a href=\"");
                    result_bytes.extend_from_slice(href_val);
                    result_bytes.extend_from_slice(b"\" data-wiki-link=\"");
                    result_bytes.extend_from_slice(target);
                    result_bytes.extend_from_slice(b"\" class=\"wiki-link\">");
                    result_bytes.extend_from_slice(title);
                    result_bytes.extend_from_slice(b"</a>");
                } else {
                    let page_name = trim_bytes(&first_part);
                    result_bytes.extend_from_slice(b"<a href=\"#\" data-wiki-link=\"");
                    result_bytes.extend_from_slice(page_name);
                    result_bytes.extend_from_slice(b"\" class=\"wiki-link\">");
                    result_bytes.extend_from_slice(page_name);
                    result_bytes.extend_from_slice(b"</a>");
                }
            } else {
                result_bytes.extend_from_slice(&binary_data[start_idx..i]);
            }
            continue;
        }

        result_bytes.push(byte);
        i += 1;
    }

    String::from_utf8_lossy(&result_bytes).into_owned()
}