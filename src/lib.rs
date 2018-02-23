mod nnet_language_identifier;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let identifier = nnet_language_identifier::NNetLanguageIdentifier::new();
        let result = identifier.find_language("吾輩は猫である");
        println!("{}", result.language);
    }
}
