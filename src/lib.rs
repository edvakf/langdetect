mod nnet_language_identifier;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lang = nnet_language_identifier::find_language("吾輩は猫である");
        println!("{}", lang);
    }
}
