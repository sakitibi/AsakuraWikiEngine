/// #comment(...) 構文をパースしてHTMLの入力フォームにする関数
pub fn parse_comment_block(line: &str, wiki_slug: &str, page_slug: &str) -> Option<String> {
    let lower = line.to_lowercase();
    if !lower.starts_with("#comment") {
        return None;
    }

    // 引数 (above / below) の抽出
    let position = if let Some(start_idx) = lower.find('(') {
        if let Some(end_idx) = lower.find(')') {
            let arg = lower[start_idx + 1..end_idx].trim();
            if arg == "below" { "below" } else { "above" }
        } else {
            "above"
        }
    } else {
        "above"
    };

    Some(format!(
        "<div class=\"commentform-container\">\
            <form class=\"wiki-comment-form\" data-wiki-slug=\"{}\" data-page-slug=\"{}\" data-position=\"{}\" style=\"margin: 1em 0;\">\
                <div style=\"margin-bottom: 8px;\">\
                    <label>\
                        名前: \
                        <input type=\"text\" name=\"name\" style=\"margin-left: 8px;\" />\
                    </label>\
                </div>\
                <div style=\"margin-bottom: 8px;\">\
                    <label style=\"display: block;\">コメント:</label>\
                    <textarea name=\"body\" required style=\"width: 100%; min-height: 60px;\"></textarea>\
                </div>\
                <button type=\"submit\" class=\"comment-submit\">\
                    <span>コメント送信</span>\
                </button>\
            </form>\
        </div>",
        wiki_slug, page_slug, position
    ))
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn test_comment_default() {
        let input = "#comment";
        let output = parse(input, "my-wiki", "home-page");
        assert!(output.contains("class=\"commentform-container\""));
        assert!(output.contains("data-wiki-slug=\"my-wiki\""));
        assert!(output.contains("data-page-slug=\"home-page\""));
        assert!(output.contains("data-position=\"above\""));
    }

    #[test]
    fn test_comment_below() {
        let input = "#comment(below)";
        let output = parse(input, "game-wiki", "stage1");
        assert!(output.contains("data-position=\"below\""));
    }
}