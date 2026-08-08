use crate::parser::parse;
use crate::parser_options::ParserOptions;
use crate::inline::parse_inline_elements;

/// アコーディオン引数の解析結果
pub struct AccordionArgs {
    pub title: String,
    pub level: String, // "*", "**", "***"
    pub is_open: bool,
}

/// #accordion の引数を解析する関数
pub fn parse_accordion_args(line: &str) -> AccordionArgs {
    // #accordion の後の文字列を取得
    let after_prefix = if let Some(pos) = line.find("#accordion") {
        line[pos + "#accordion".len()..].trim()
    } else {
        line.trim()
    };

    // {{ の前までのパラメータ部分を抽出
    let params_str = if let Some(idx) = after_prefix.find("{{") {
        &after_prefix[..idx]
    } else {
        after_prefix
    };

    let params_str = params_str.trim();

    // カッコ () がある場合は中身を取り出す
    let raw_args = if params_str.starts_with('(') && params_str.contains(')') {
        let start = params_str.find('(').unwrap() + 1;
        let end = params_str.rfind(')').unwrap();
        &params_str[start..end]
    } else {
        params_str
    };

    // カンマ区切りでパース
    let args: Vec<&str> = raw_args.split(',').map(|s| s.trim()).collect();

    let title = args.get(0).copied().unwrap_or("").to_string();

    // レベル (*, **, ***) の判定
    let level = args
        .iter()
        .find(|&&a| a == "*" || a == "**" || a == "***")
        .copied()
        .unwrap_or("*")
        .to_string();

    // open フラグの判定
    let is_open = args.iter().any(|&a| a.eq_ignore_ascii_case("open"));

    AccordionArgs {
        title,
        level,
        is_open,
    }
}

pub fn render_accordion_block(
    args: &AccordionArgs,
    content_lines: &[&str],
    wiki_slug: &str,
    page_slug: &str,
    options: &ParserOptions,
) -> String {
    let tag = match args.level.as_str() {
        "***" => "h4",
        "**" => "h3",
        _ => "h2",
    };

    let parsed_title = parse_inline_elements(&args.title, args.title.as_bytes(), options);
    let display_style = if args.is_open { "block" } else { "none" };

    let inner_body = content_lines.join("\n");
    let parsed_content = parse(&inner_body, inner_body.as_bytes(), wiki_slug, page_slug, options);

    let icon_path = if args.is_open {
        "M384 32H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64h320c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64zM320 272H128c-13.3 0-24-10.7-24-24s10.7-24 24-24h192c13.3 0 24 10.7 24 24s-10.7 24-24 24z"
    } else {
        "M64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM200 344l0-64-64 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l64 0 0-64c0-13.3 10.7-24 24-24s24 10.7 24 24l0 64 64 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-64 0 0 64c0 13.3-10.7 24-24 24s-24-10.7-24-24z"
    };

    format!(
        r#"<div class="accordion-container">
<{tag} class="accordion-header">
    <svg aria-hidden="true" focusable="false" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" style="width: 1em; height: 1em;">
        <path fill="currentColor" d="{icon_path}" />
    </svg>
    {parsed_title}
</{tag}>
<div class="accordion-content" style="display: {display_style};">
{parsed_content}
</div>
</div>"#,
        tag = tag,
        icon_path = icon_path,
        parsed_title = parsed_title,
        display_style = display_style,
        parsed_content = parsed_content
    )
}