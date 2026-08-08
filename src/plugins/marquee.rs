use crate::parser_options::ParserOptions;
use crate::inline::parse_inline_elements;

pub fn parse_marquee_block(line: &str, options: &ParserOptions) -> Option<String> {
    let start_idx = line.to_lowercase().find("#marquee(")? + "#marquee(".len();
    let chars: Vec<char> = line[start_idx..].chars().collect();
    
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut found_close = false;

    for &c in &chars {
        if c == ',' {
            args.push(current_arg.trim().to_string());
            current_arg = String::new();
        } else if c == ')' {
            args.push(current_arg.trim().to_string());
            found_close = true;
            break;
        } else {
            current_arg.push(c);
        }
    }

    if !found_close {
        return None;
    }

    let text = args.get(0).cloned().unwrap_or_default();
    let slide = args.get(1).cloned().unwrap_or_default();       
    let loop_val = args.get(2).cloned().unwrap_or_default();    
    let bg_color = args.get(3).cloned().unwrap_or_default();    
    let color = args.get(4).cloned().unwrap_or_default();       
    let size = args.get(5).cloned().unwrap_or_default();        

    let font_size = if !size.is_empty() { format!("{}px", size) } else { "inherit".to_string() };
    let iteration_count = if !loop_val.is_empty() && loop_val.chars().all(|c| c.is_ascii_digit()) {
        loop_val
    } else {
        "infinite".to_string()
    };

    let animation_base = match slide.as_str() {
        "slide" => "scroll-once",
        "alternate" => "scroll-alternate",
        _ => "scroll-default",
    };

    let size_suffix = "xl"; 
    let animation_name = format!("{}-{}", animation_base, size_suffix);

    let duration = match slide.as_str() {
        "slide" => "5s",
        "alternate" => "7s",
        _ => "15s",
    };

    let timing_function = match slide.as_str() {
        "slide" | "alternate" => "ease-in-out",
        _ => "linear",
    };

    let direction = match slide.as_str() {
        "alternate" => "alternate",
        _ => "normal",
    };

    let fill_mode = match slide.as_str() {
        "slide" => "forwards",
        _ => "none",
    };

    let bg_style = if !bg_color.is_empty() { bg_color } else { "transparent".to_string() };
    let color_style = if !color.is_empty() { color } else { "inherit".to_string() };

    let processed_text = parse_inline_elements(&text, text.as_bytes(), options);

    Some(format!(
        "<div style=\"overflow: hidden; white-space: nowrap; background-color: {}; color: {}; font-size: {};\">\
            <div data-animation-base=\"{}\" style=\"animation-name: {}; animation-duration: {}; animation-timing-function: {}; animation-iteration-count: {}; animation-direction: {}; animation-fill-mode: {}; display: inline-block;\">\
                {}\
            </div>\
         </div>",
        bg_style, color_style, font_size,
        animation_base, animation_name, duration, timing_function, iteration_count, direction, fill_mode,
        processed_text
    ))
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::parser_options::ParserOptions;

    #[test]
    fn test_marquee_default() {
        let input = "#marquee(こんにちは,scroll,infinite,#ff0000,#ffffff,20)";
        let options = ParserOptions::default();
        let output = parse(input, input.as_bytes(), "test-wiki", "test-page", &options);
        
        assert!(output.contains("data-animation-base=\"scroll-default\""));
        assert!(output.contains("animation-name: scroll-default-xl"));
        assert!(output.contains("animation-duration: 15s"));
        assert!(output.contains("animation-timing-function: linear"));
        assert!(output.contains("background-color: #ff0000"));
        assert!(output.contains("color: #ffffff"));
        assert!(output.contains("font-size: 20px"));
    }

    #[test]
    fn test_marquee_slide_once() {
        let input = "#marquee(一度だけ流れる,slide,1,transparent,inherit,)";
        let options = ParserOptions::default();
        let output = parse(input, input.as_bytes(), "test-wiki", "test-page", &options);
        
        assert!(output.contains("data-animation-base=\"scroll-once\""));
        assert!(output.contains("animation-name: scroll-once-xl"));
        assert!(output.contains("animation-duration: 5s"));
        assert!(output.contains("animation-timing-function: ease-in-out"));
        assert!(output.contains("animation-iteration-count: 1"));
        assert!(output.contains("animation-fill-mode: forwards"));
    }

    #[test]
    fn test_marquee_alternate() {
        let input = "#MARQUEE( 往復テキスト , alternate , infinite )";
        let options = ParserOptions::default();
        let output = parse(input, input.as_bytes(), "test-wiki", "test-page", &options);
        
        assert!(output.contains("data-animation-base=\"scroll-alternate\""));
        assert!(output.contains("animation-name: scroll-alternate-xl"));
        assert!(output.contains("animation-duration: 7s"));
        assert!(output.contains("animation-direction: alternate"));
        assert!(output.contains("往復テキスト")); 
    }
}