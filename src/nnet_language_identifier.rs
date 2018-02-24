use task_context_params;

const UNKNOWN: &str = "und";
const NUM_SNIPPETS: i32 = 5;

pub struct NNetLanguageIdentifier {
    num_languages: i32,
    min_num_bytes: i32,
    max_num_bytes: i32,
    num_snippets: i32,
    snippet_size: i32
}

impl NNetLanguageIdentifier {
    pub fn new(min_num_bytes: i32, max_num_bytes: i32) -> Self {
        assert!(max_num_bytes > 0);
        assert!(min_num_bytes >= 0);
        assert!(min_num_bytes < max_num_bytes);

        // TODO: num_languages, network
        let num_languages = task_context_params::GetNumLanguages();

        let num_snippets = if max_num_bytes <= NUM_SNIPPETS { 1 } else { NUM_SNIPPETS };
        let snippet_size = max_num_bytes / num_snippets;

        // TODO: WholeSentenceFeature

        // TODO: TaskContext

        return Self{
            num_languages: num_languages,
            min_num_bytes: min_num_bytes,
            max_num_bytes: max_num_bytes,
            num_snippets: num_snippets,
            snippet_size: snippet_size
        };
    }

    #[allow(dead_code)]
    pub fn find_language(self, text: &str) -> Result {

        // TODO: check valid utf-8 sequence
        // TODO: strip HTML tags
        let t = truncate_to_at_most_n_bytes(text, 2);

        // lower case

        // CheapSqueezeInplace of text


        println!("{}", t);
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

fn truncate_to_at_most_n_bytes(s: &str, n: usize) -> String {
    let chars: String = s
        .char_indices()
        .take_while(|&(i, c)| i + c.len_utf8() <= n)
        .map(|(_, c)| c)
        .collect();
    return chars;
}

#[cfg(test)]
mod tests {
    use super::truncate_to_at_most_n_bytes;

    #[test]
    fn test_truncate_to_at_most_n_bytes() {
        assert_eq!(truncate_to_at_most_n_bytes("abcde", 0), String::from(""));
        assert_eq!(truncate_to_at_most_n_bytes("abcde", 1), String::from("a"));
        assert_eq!(truncate_to_at_most_n_bytes("abcde", 2), String::from("ab"));
        assert_eq!(truncate_to_at_most_n_bytes("abcde", 10), String::from("abcde"));
        assert_eq!(truncate_to_at_most_n_bytes("吾輩は猫である", 0), String::from(""));
        assert_eq!(truncate_to_at_most_n_bytes("吾輩は猫である", 1), String::from(""));
        assert_eq!(truncate_to_at_most_n_bytes("吾輩は猫である", 2), String::from(""));
        assert_eq!(truncate_to_at_most_n_bytes("吾輩は猫である", 3), String::from("吾"));
        assert_eq!(truncate_to_at_most_n_bytes("吾輩は猫である", 4), String::from("吾"));
        assert_eq!(truncate_to_at_most_n_bytes("吾輩は猫である", 5), String::from("吾"));
        assert_eq!(truncate_to_at_most_n_bytes("吾輩は猫である", 6), String::from("吾輩"));
    }
}
