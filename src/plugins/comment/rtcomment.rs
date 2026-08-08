use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::{Deserialize, Serialize};
use crate::env::{SUPABASE_URL, SUPABASE_KEY};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentInput {
    pub name: String,
    pub body: String,
    pub wiki_slug: String,
    pub page_slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

pub fn parse_rtcomment_block(line: &str, wiki_slug: &str, page_slug: &str) -> Option<String> {
    let trimmed = line.trim();
    let lower_trimmed = trimmed.to_lowercase();
    
    if lower_trimmed == "#rtcomment" || lower_trimmed == "#rtcomment()" {
        Some(format!(
            "<div class=\"wiki-rtcomment\" data-wiki-slug=\"{}\" data-page-slug=\"{}\"></div>",
            wiki_slug, page_slug
        ))
    } else {
        None
    }
}

#[wasm_bindgen]
pub async fn fetch_comments_wasm(
    wiki_slug: &str,
    page_slug: &str,
) -> Result<String, JsValue> {
    let url = format!(
        "{}/rest/v1/comments?wiki_slug=eq.{}&page_slug=eq.{}&order=created_at.asc",
        SUPABASE_URL, wiki_slug, page_slug
    );

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    
    let headers = request.headers();
    headers.set("apikey", SUPABASE_KEY)?;
    headers.set("Authorization", &format!("Bearer {}", SUPABASE_KEY))?;

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window object"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(JsValue::from_str(&format!("HTTP error! status: {}", resp.status())));
    }

    let json_promise = resp.text()?;
    let text = JsFuture::from(json_promise).await?;
    
    text.as_string().ok_or_else(|| JsValue::from_str("Failed to get response body text"))
}

#[wasm_bindgen]
pub async fn send_comment_wasm(
    wiki_slug: &str,
    page_slug: &str,
    name: &str,
    body: &str,
    user_id: Option<String>,
) -> Result<bool, JsValue> {
    let url = format!("{}/rest/v1/comments", SUPABASE_URL);

    let payload = CommentInput {
        name: name.to_string(),
        body: body.to_string(),
        wiki_slug: wiki_slug.to_string(),
        page_slug: page_slug.to_string(),
        user_id,
    };

    let json_body = serde_json::to_string(&payload)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&json_body));

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let headers = request.headers();
    headers.set("apikey", SUPABASE_KEY)?;
    headers.set("Authorization", &format!("Bearer {}", SUPABASE_KEY))?;
    headers.set("Content-Type", "application/json")?;
    headers.set("Prefer", "return=minimal")?;

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window object"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    Ok(resp.ok())
}