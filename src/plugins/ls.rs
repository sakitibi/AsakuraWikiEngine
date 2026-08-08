use regex::Regex;

/// #ls および #ls2 構文をパースしてHTMLプレースホルダーを生成する関数
pub fn parse_ls_block(line: &str, wiki_slug: &str) -> Option<String> {
    let trimmed = line.trim();
    let lower = trimmed.to_lowercase();

    if lower.starts_with("#ls2") {
        let re_ls2 = Regex::new(
            r"(?i)^#ls2\(\s*([^[\],]+)(?:\[\s*([^\]]+)\s*\])?(?:,\s*\{\s*([^}]+)\s*\})?(?:,\s*([^)]+))?\)"
        ).ok()?;

        if let Some(caps) = re_ls2.captures(trimmed) {
            let pattern = caps.get(1).map_or("", |m| m.as_str().trim());
            // オプションは [ ] 形式 または { } 形式のいずれかから取得
            let options = caps.get(2)
                .or_else(|| caps.get(3))
                .map_or("", |m| m.as_str().trim());
            let label = caps.get(4).map_or("", |m| m.as_str().trim());

            return Some(format!(
                r#"<div class="wiki-pagelist2" data-wiki-slug="{}" data-pattern="{}" data-options="{}" data-label="{}"></div>"#,
                wiki_slug, pattern, options, label
            ));
        }
    }

    // 2. #ls 構文のパース: #ls または #ls(title)
    if lower.starts_with("#ls") {
        let re_ls = Regex::new(r"(?i)^#ls(?:\(([^)]+)\))?").ok()?;

        if let Some(caps) = re_ls.captures(trimmed) {
            let prefix = caps.get(1).map_or("", |m| m.as_str().trim());

            return Some(format!(
                r#"<div class="wiki-pagelist" data-wiki-slug="{}" data-prefix="{}"></div>"#,
                wiki_slug, prefix
            ));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ls_default() {
        let result = parse_ls_block("#ls", "sample-wiki").unwrap();
        assert_eq!(
            result,
            r#"<div class="wiki-pagelist" data-wiki-slug="sample-wiki" data-prefix=""></div>"#
        );
    }

    #[test]
    fn test_parse_ls_with_prefix() {
        let result = parse_ls_block("#ls(title)", "sample-wiki").unwrap();
        assert_eq!(
            result,
            r#"<div class="wiki-pagelist" data-wiki-slug="sample-wiki" data-prefix="title"></div>"#
        );
    }

    #[test]
    fn test_parse_ls2_full() {
        let result = parse_ls_block("#ls2(Folder/ [title, compact], 一覧はこちら)", "sample-wiki").unwrap();
        assert_eq!(
            result,
            r#"<div class="wiki-pagelist2" data-wiki-slug="sample-wiki" data-pattern="Folder/" data-options="title, compact" data-label="一覧はこちら"></div>"#
        );
    }
}