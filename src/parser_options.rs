use serde::Deserialize;

/// パースモード（テキストまたはバイナリ）
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParseMode {
    Text,
    Binary,
}

impl Default for ParseMode {
    fn default() -> Self {
        ParseMode::Text
    }
}

/// パース有効・無効を制御するフラグ構造体
#[derive(Deserialize, Debug, Clone)]
pub struct ParserOptions {
    #[serde(default)]
    pub mode: ParseMode,

    #[serde(rename = "CENTER", default = "default_true")]
    pub center: bool,
    #[serde(rename = "LEFT", default = "default_true")]
    pub left: bool,
    #[serde(rename = "RIGHT", default = "default_true")]
    pub right: bool,
    #[serde(rename = "#hr", default = "default_true")]
    pub hr: bool,
    #[serde(rename = "#br", default = "default_true")]
    pub br_block: bool,
    #[serde(rename = "&br;", default = "default_true")]
    pub br_inline: bool,
    #[serde(rename = "#accordion", default = "default_true")]
    pub accordion: bool,
    #[serde(rename = "#calendar", default = "default_true")]
    pub calendar: bool,
    #[serde(rename = "#comment", default = "default_true")]
    pub comment: bool,
    #[serde(rename = "#rtcomment", default = "default_true")]
    pub rtcomment: bool,
    #[serde(rename = "#marquee", default = "default_true")]
    pub marquee: bool,
    #[serde(rename = "#include", default = "default_true")]
    pub include: bool,
    #[serde(rename = "#ls", default = "default_true")]
    pub ls: bool,
    #[serde(rename = "#const", default = "default_true")]
    pub const_block: bool,
    #[serde(rename = "#let", default = "default_true")]
    pub let_block: bool,
}

fn default_true() -> bool {
    true
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            mode: ParseMode::Text,
            center: true,
            left: true,
            right: true,
            hr: true,
            br_block: true,
            br_inline: true,
            accordion: true,
            calendar: true,
            comment: true,
            rtcomment: true,
            marquee: true,
            include: true,
            ls: true,
            const_block: true,
            let_block: true,
        }
    }
}