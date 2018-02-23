const UNKNOWN: &str = "und";
const NUM_SNIPPETS: i32 = 5;

pub struct NNetLanguageIdentifier {
    min_num_bytes: i32,
    max_num_bytes: i32,
    num_snippets: i32,
    snippet_size: i32
}

impl NNetLanguageIdentifier {
    pub fn new(min_num_bytes: i32, max_num_bytes: i32) -> Self {
        let num_snippets = if max_num_bytes <= NUM_SNIPPETS { 1 } else { NUM_SNIPPETS };
        let snippet_size = max_num_bytes / num_snippets;

        return Self{
            min_num_bytes: min_num_bytes,
            max_num_bytes: max_num_bytes,
            num_snippets: num_snippets,
            snippet_size: snippet_size
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
