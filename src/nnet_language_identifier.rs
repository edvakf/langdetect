
pub fn find_language(text: &str) -> Result {
    println!("{}", text);
    return Result{
        language: String::from("ja"),
        ..Default::default()
    };
}

pub struct Result {
    pub language: String, // TODO: default value
    pub probability: f64,
    pub is_reliable: bool,
    pub proportion: f64
}

impl Default for Result {
    fn default() -> Result {
        Result {
            language: "unknown".to_string(), // TODO: kUnknown
            probability: 0.0,
            is_reliable: false,
            proportion: 0.0
        }
   }
}
