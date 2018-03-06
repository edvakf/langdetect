mod nnet_language_identifier;
mod task_context;
mod task_context_params;
mod language_identifier_features;
mod feature_extractor;
mod utils;
mod feature_types;
mod sentence_features;
mod embedding_feature_extractor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let identifier = nnet_language_identifier::NNetLanguageIdentifier::new(140, 700);
        let result = identifier.find_language("吾輩は猫である");
        println!("{}", result.language);
    }
}
