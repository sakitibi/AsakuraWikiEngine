use crate::parser_options::ParserOptions;

pub fn parse_include_block(line: &str, _options: &ParserOptions) -> Option<String> {
    let trimmed = line.trim();
    let lower_trimmed = trimmed.to_lowercase();

    if !lower_trimmed.starts_with("#include") {
        return None;
    }

    let start_idx = trimmed.find('(')?;
    let end_idx = trimmed.rfind(')')?;
    if start_idx >= end_idx {
        return None;
    }

    let args_str = &trimmed[start_idx + 1..end_idx];
    let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

    let first_arg = args.get(0).copied().unwrap_or("");
    if first_arg.is_empty() {
        return None;
    }

    let mut page_name = first_arg;
    let mut stylesheet_url = "";
    if first_arg.contains('|') {
        let parts: Vec<&str> = first_arg.splitn(2, '|').map(|s| s.trim()).collect();
        page_name = parts[0];
        stylesheet_url = parts[1];
    }

    let line_range = args.get(1).copied().unwrap_or("");
    if !line_range.is_empty() && !is_valid_line_range(line_range) {
        return Some("<div style=\"color: red;\">読み込み失敗: 無効な行範囲です</div>".to_string());
    }

    let flag = args.get(2).copied().unwrap_or("").to_lowercase();
    let show_title = match flag.as_str() {
        "notitle" | "none" => "false",
        "title" => "true",
        _ => "default",
    };

    Some(format!(
        "<div class=\"wiki-include-page\" data-page=\"{}\" data-stylesheet=\"{}\" data-range=\"{}\" data-show-title=\"{}\"></div>",
        page_name, stylesheet_url, line_range, show_title
    ))
}

fn is_valid_line_range(range: &str) -> bool {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        return false;
    }
    let start_ok = parts[0].trim().parse::<u32>().is_ok();
    let end_ok = parts[1].trim().parse::<u32>().is_ok();
    start_ok && end_ok
}