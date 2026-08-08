use crate::calendar::parse_calendar_block;
use crate::comment::parse_comment_block;
use crate::ls::parse_ls_block;
use crate::marquee::parse_marquee_block;
use crate::rtcomment::parse_rtcomment_block;
use crate::accordion::{parse_accordion_args, render_accordion_block};
use crate::include::parse_include_block;
use crate::variable::{parse_const_block, parse_let_block, parse_variable_inline, VariableContext};
use crate::inline::parse_inline_elements;
use crate::parser_options::{ParserOptions};

pub fn parse(
    input: &str,
    bytes: &[u8],
    wiki_slug: &str,
    page_slug: &str,
    options: &ParserOptions,
) -> String {
    let mut result = String::new();

    let lines: Vec<&str> = input.lines().collect();
    let byte_lines: Vec<&[u8]> = bytes
        .split(|&b| b == b'\n')
        .map(|line| if line.ends_with(b"\r") { &line[..line.len() - 1] } else { line })
        .collect();

    let mut i = 0;
    let mut var_ctx = VariableContext::new();

    while i < lines.len() {
        let line = lines[i];
        let line_bytes = if i < byte_lines.len() { byte_lines[i] } else { b"" as &[u8] };

        let mut processed_line = line.to_string();
        let mut text_align = "";

        // 行頭の文字寄せ指定のチェック
        if options.center && processed_line.starts_with("CENTER:") {
            text_align = "center";
            processed_line = processed_line["CENTER:".len()..].to_string();
        } else if options.left && processed_line.starts_with("LEFT:") {
            text_align = "left";
            processed_line = processed_line["LEFT:".len()..].to_string();
        } else if options.right && processed_line.starts_with("RIGHT:") {
            text_align = "right";
            processed_line = processed_line["RIGHT:".len()..].to_string();
        }

        let lower_line = processed_line.trim().to_lowercase();

        // 1. #accordion ブロック判定
        if options.accordion && lower_line.starts_with("#accordion") && processed_line.contains("{{") {
            let mut accordion_lines = Vec::new();
            let mut open_brackets_count = 0;
            let mut found_end = false;
            let start_line_idx = i;

            while i < lines.len() {
                let current_line = lines[i];
                accordion_lines.push(current_line);

                let open_count = current_line.matches('{').count();
                let close_count = current_line.matches('}').count();
                open_brackets_count = open_brackets_count + open_count as i32 - close_count as i32;

                if open_brackets_count <= 0 && i > start_line_idx {
                    found_end = true;
                    break;
                }
                i += 1;
            }

            if found_end && !accordion_lines.is_empty() {
                let first_line = accordion_lines[0];
                let args = parse_accordion_args(first_line);
                let content_lines = &accordion_lines[1..accordion_lines.len() - 1];
                
                let accordion_html = render_accordion_block(
                    &args, 
                    content_lines, 
                    wiki_slug, 
                    page_slug, 
                    options
                );

                result.push_str(&accordion_html);
                result.push('\n');
                i += 1;
                continue;
            }
            i = start_line_idx;
            processed_line = line.to_string();
        }

        // 2. #include ブロック判定
        if options.include && lower_line.starts_with("#include") {
            if let Some(include_html) = parse_include_block(&processed_line, options) {
                result.push_str(&include_html);
                result.push('\n');
                i += 1;
                continue;
            } else {
                processed_line = parse_inline_elements(&processed_line, line_bytes, options);
            }
        }
        // 3. #ls / #ls2 ブロック判定
        else if options.ls && (lower_line.starts_with("#ls") || lower_line.starts_with("#ls2")) {
            if let Some(ls_html) = parse_ls_block(&processed_line, wiki_slug) {
                result.push_str(&ls_html);
                result.push('\n');
                i += 1;
                continue;
            } else {
                processed_line = parse_inline_elements(&processed_line, line_bytes, options);
            }
        }
        // 4. #const ブロック判定
        else if options.const_block && lower_line.starts_with("#const") {
            if let Some(const_html) = parse_const_block(&processed_line, &mut var_ctx) {
                result.push_str(&const_html);
                result.push('\n');
                i += 1;
                continue;
            } else {
                processed_line = parse_inline_elements(&processed_line, line_bytes, options);
            }
        }
        // 5. #let ブロック判定
        else if options.let_block && lower_line.starts_with("#let") {
            if let Some(let_html) = parse_let_block(&processed_line, &mut var_ctx) {
                result.push_str(&let_html);
                result.push('\n');
                i += 1;
                continue;
            } else {
                processed_line = parse_inline_elements(&processed_line, line_bytes, options);
            }
        }
        // 6. その他のブロック要素の判定 (#hr, #br)
        else if options.hr && lower_line == "#hr" {
            processed_line = "<hr>".to_string();
        } else if options.br_block && lower_line == "#br" {
            processed_line = "<br>".to_string();
        } 
        else if options.calendar && lower_line.starts_with("#calendar") {
            if let Some(cal_html) = parse_calendar_block(&processed_line) {
                result.push_str(&cal_html);
                result.push('\n');
                i += 1;
                continue;
            } else {
                processed_line = parse_inline_elements(&processed_line, line_bytes, options);
            }
        } 
        else if options.comment && lower_line.starts_with("#comment") {
            if let Some(cmt_html) = parse_comment_block(&processed_line, wiki_slug, page_slug) {
                result.push_str(&cmt_html);
                result.push('\n');
                i += 1;
                continue;
            } else {
                processed_line = parse_inline_elements(&processed_line, line_bytes, options);
            }
        } else if options.rtcomment && lower_line.starts_with("#rtcomment") {
            if let Some(rt_html) = parse_rtcomment_block(&processed_line, wiki_slug, page_slug) {
                processed_line = rt_html;
            } else {
                processed_line = parse_inline_elements(&processed_line, line_bytes, options);
            }
        } 
        else if options.marquee && lower_line.starts_with("#marquee") {
            if let Some(mq_html) = parse_marquee_block(&processed_line, options) {
                processed_line = mq_html;
            } else {
                processed_line = parse_inline_elements(&processed_line, line_bytes, options);
            }
        } else {
            // ブロック要素に当てはまらない場合、行ごとのインライン要素パースを実行
            processed_line = parse_inline_elements(&processed_line, line_bytes, options);
        }

        // 基本的なインライン解析が終わった後に、変数・定数置換を展開
        processed_line = parse_variable_inline(&processed_line, &mut var_ctx);

        // 文字寄せDivラップ処理
        if !text_align.is_empty() {
            result.push_str(&format!("<div style=\"text-align: {};\">{}</div>", text_align, processed_line));
        } else {
            result.push_str(&processed_line);
        }

        if i < lines.len() - 1 {
            result.push_str("<br>");
        }

        i += 1;
    }

    result
}