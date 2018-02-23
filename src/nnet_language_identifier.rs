const UNKNOWN: &str = "und";

pub struct NNetLanguageIdentifier {
    min_num_bytes: i32,
    max_num_bytes: i32
}

impl NNetLanguageIdentifier {
    pub fn new(min_num_bytes: i32, max_num_bytes: i32) -> Self {
        return Self{
            min_num_bytes: min_num_bytes,
            max_num_bytes: max_num_bytes
        };
    }

    #[allow(dead_code)]
    pub fn find_language(self, text: &str) -> Result {
        println!("{}", text);
        return Result{
            language: String::from("ja"),
            ..Default::default()
        };
    }
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
