const UNKNOWN: &str = "und";

#[allow(dead_code)]
pub fn find_language(text: &str) -> Result {
    println!("{}", text);
    return Result{
        language: String::from("ja"),
        ..Default::default()
    };
}

pub struct Result {
    pub language: String,
    pub probability: f64,
    pub is_reliable: bool,
    pub proportion: f64
}

impl Default for Result {
    fn default() -> Result {
        Result {
            language: UNKNOWN.to_string(),
            probability: 0.0,
            is_reliable: false,
            proportion: 0.0
        }
   }
}
