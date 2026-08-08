use std::collections::HashMap;
use regex::Regex;

#[derive(Default, Clone, Debug)]
pub struct VariableContext {
    pub const_context: HashMap<String, String>,
    pub let_context: HashMap<String, String>,
}

impl VariableContext {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn parse_const_block(line: &str, context: &mut VariableContext) -> Option<String> {
    // 割と厳密な正規表現
    let re = Regex::new(r"(?i)^#const\(\s*([^:]+?)\s*:\s*([^)]+?)\s*\)\{([^\}]+?)\};").ok()?;
    let caps = re.captures(line.trim())?;

    let var_name = caps.get(1)?.as_str().trim().to_string();
    let var_type = caps.get(2)?.as_str().trim();
    let var_value = caps.get(3)?.as_str().trim().to_string();

    if context.const_context.contains_key(&var_name) {
        return Some(format!(
            r#"<span style="color: red;">定数 {} は再定義不可！</span>"#,
            var_name
        ));
    }

    context.const_context.insert(var_name.clone(), var_value.clone());
    
    Some(format!(
        r#"<span style="display: none; font-weight: bold;">定数 {}（{}） = {}</span>"#,
        var_name, var_type, var_value
    ))
}

pub fn parse_let_block(line: &str, context: &mut VariableContext) -> Option<String> {
    let re = Regex::new(r"(?i)^#let\(\s*([^:]+?)\s*:\s*([^)]+?)\s*\)\{([^\}]+?)\};").ok()?;
    let caps = re.captures(line.trim())?;

    let var_name = caps.get(1)?.as_str().trim().to_string();
    let var_type = caps.get(2)?.as_str().trim();
    let var_value = caps.get(3)?.as_str().trim().to_string();

    context.let_context.insert(var_name.clone(), var_value.clone());

    Some(format!(
        r#"<span style="display: none; font-style: italic;">変数 {}（{}） ← {}</span>"#,
        var_name, var_type, var_value
    ))
}

pub fn parse_variable_inline(text: &str, context: &mut VariableContext) -> String {
    let mut result = text.to_string();

    if let Ok(re) = Regex::new(r"(?i)&const-use\(([^)]+?)\);") {
        result = re.replace_all(&result, |caps: &regex::Captures| {
            let var_name = caps.get(1).unwrap().as_str().trim();
            context.const_context.get(var_name)
                .cloned()
                .unwrap_or_else(|| format!("[定数未定義:{}]", var_name))
        }).to_string();
    }

    if let Ok(re) = Regex::new(r"(?i)&let-use\(([^)]+?)\);") {
        result = re.replace_all(&result, |caps: &regex::Captures| {
            let var_name = caps.get(1).unwrap().as_str().trim();
            context.let_context.get(var_name)
                .cloned()
                .unwrap_or_else(|| format!("[変数未定義:{}]", var_name))
        }).to_string();
    }

    if let Ok(re) = Regex::new(r"(?i)&relet\(([^)]+?)\);") {
        result = re.replace_all(&result, |caps: &regex::Captures| {
            let var_name = caps.get(1).unwrap().as_str().trim();
            if context.let_context.contains_key(var_name) {
                format!(r#"<span style="display: none;">再代入OK: {}</span>"#, var_name)
            } else {
                format!(r#"<span style="color: red;">再代入対象 `{}` が未定義です</span>"#, var_name)
            }
        }).to_string();
    }

    result
}