use chrono::{Datelike, NaiveDate, Weekday};

/// #calendar2(...) の中身をパースしてHTMLの<table>を組み立てる関数
pub fn parse_calendar_block(line: &str) -> Option<String> {
    let start_idx = line.to_lowercase().find("#calendar2(")? + "#calendar2(".len();
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

    let (year, month, is_off) = if args.len() >= 1 {
        let first_arg = &args[0];
        
        if args.len() >= 2 && first_arg.len() == 4 && args[1].len() == 2 {
            let y: i32 = first_arg.parse().ok()?;
            let m: u32 = args[1].parse().ok()?;
            let off = args.get(2).map(|s| s.to_lowercase() == "off").unwrap_or(false);
            (y, m, off)
        } 
        else if first_arg.len() >= 6 {
            let y_str = &first_arg[0..4];
            let m_str = &first_arg[4..6];
            let y: i32 = y_str.parse().ok()?;
            let m: u32 = m_str.parse().ok()?;
            let off = args.get(1).map(|s| s.to_lowercase() == "off").unwrap_or(false);
            (y, m, off)
        } else {
            return None;
        }
    } else {
        return None;
    };

    if month < 1 || month > 12 {
        return None;
    }

    let first_day_date = NaiveDate::from_ymd_opt(year, month, 1)?;
    let first_day_of_week = first_day_date.weekday().num_days_from_sunday();

    let next_month_date = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)?
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)?
    };
    let days_in_month = (next_month_date - first_day_date).num_days() as u32;

    let mut tbody_html = String::new();
    tbody_html.push_str("<tr>");

    let mut current_column_count = 0;

    for _ in 0..first_day_of_week {
        tbody_html.push_str("<td></td>");
        current_column_count += 1;
    }

    for d in 1..=days_in_month {
        if current_column_count == 7 {
            tbody_html.push_str("</tr><tr>");
            current_column_count = 0;
        }

        let current_date = NaiveDate::from_ymd_opt(year, month, d)?;
        let weekday = current_date.weekday();
        
        let cls = if is_off {
            ""
        } else {
            match weekday {
                Weekday::Sun => " class=\"sunday\"",
                Weekday::Sat => " class=\"saturday\"",
                _ => "",
            }
        };

        tbody_html.push_str(&format!("<td{}>{}</td>", cls, d));
        current_column_count += 1;
    }

    while current_column_count < 7 {
        tbody_html.push_str("<td></td>");
        current_column_count += 1;
    }
    tbody_html.push_str("</tr>");

    Some(format!(
        "<table class=\"calendar2\">\
        <thead>\
            <tr>\
                <th>日</th><th>月</th><th><span style=\"color: red;\">火</span></th>\
                <th><span style=\"color: red;\">水</span></th><th>木</th><th>金</th><th>土</th>\
            </tr>\
        </thead>\
        <tbody>\
            {}\
        </tbody>\
        </table>",
        tbody_html
    ))
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn test_calendar2_normal() {
        let input = "#calendar2(2026, 07)";
        let output = parse(input, "test-wiki", "test-page");
        assert!(output.contains("<table class=\"calendar2\">"));
        assert!(output.contains("class=\"sunday\""));
    }

    #[test]
    fn test_calendar2_joined() {
        let input = "#calendar2(202607)";
        let output = parse(input, "test-wiki", "test-page");
        assert!(output.contains("class=\"sunday\""));
    }

    #[test]
    fn test_calendar2_off() {
        let input = "#calendar2(202607, off)";
        let output = parse(input, "test-wiki", "test-page");
        assert!(!output.contains("class=\"sunday\""));
        assert!(!output.contains("class=\"saturday\""));
    }
}