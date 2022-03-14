use cached::proc_macro::cached;
use regex::Regex;

#[cached]
fn date_regex() -> Regex {
    const re: &'static str = r"\d{4}-\d{2}-\d{2}";
    Regex::new(re).expect("Compilation failure")
}

let test_str = r#"
    today is 2021-02-16
    tomorrow is 2021-02-18
    yesterday was 2021-02-16
"#;

if date_regex().is_match(test_str) {

}
if let Some(date) = date_regex().find(test_str) {

}
for date in date_regex().find(test_str) {
    
}