use language_identifier_features;
use sentence_features;
use script_detector;
use feature_extractor;
use feature_types;

pub struct RelevantScriptFeature {
    feature_type: language_identifier_features::NumericFeatureType,
}

impl RelevantScriptFeature {
    pub fn new(name: &'static str) -> RelevantScriptFeature {
        let feature_type = language_identifier_features::NumericFeatureType::new(
            name,
            script_detector::Script::NumRelevantScripts as feature_extractor::FeatureValue,
        );
        return RelevantScriptFeature {
            feature_type: feature_type,
        };
    }
}

impl sentence_features::WholeSentenceFeature for RelevantScriptFeature {
    fn feature_type(&self) -> &language_identifier_features::NumericFeatureType {
        &self.feature_type
    }

    fn evaluate(&self, sentence: String) -> feature_extractor::FeatureVector {
        let mut counts: [i64; script_detector::Script::NumRelevantScripts as usize] =
            [0; script_detector::Script::NumRelevantScripts as usize];
        let mut total_count = 0;

        for c in sentence.chars() {
            if c.is_ascii_alphabetic() {
                continue;
            }
            counts[script_detector::get_script(c) as usize] += 1;
            total_count += 1;
        }

        let mut feature_vector = feature_extractor::FeatureVector::new();

        for script_id in 0..(script_detector::Script::NumRelevantScripts as usize) {
            let count = counts[script_id];
            if count > 0 {
                let weight = (count as f32) / (total_count as f32);

                let feature_value =
                    feature_extractor::FloatFeatureValue::new(script_id as u32, weight);

                feature_vector.push((
                    self.feature_type() as &feature_types::FeatureType,
                    feature_value.discrete_value(),
                ));
            }
        }
        return feature_vector;
    }
}

#[cfg(test)]
mod tests {
    use super::RelevantScriptFeature;
    use super::sentence_features::WholeSentenceFeature;
    use super::feature_extractor;
    use super::script_detector;

    #[test]
    fn test_evaluate() {
        let f = RelevantScriptFeature::new("continuous-bag-of-relevant-scripts");
        assert_eq!(f.evaluate("abc".to_string()).len(), 0); // alphabetic characters do not count!

        {
            let feature_vector = f.evaluate("あいうえお".to_string());
            assert_eq!(feature_vector.len(), 1);
            assert_eq!(
                feature_vector[0].1,
                feature_extractor::FloatFeatureValue::new(
                    script_detector::Script::ScriptHiragana as u32,
                    1.0
                ).discrete_value()
            );
        }

        {
            let feature_vector = f.evaluate("ドラえもん".to_string());
            assert_eq!(feature_vector.len(), 2);
            assert_eq!(
                feature_vector[0].1,
                feature_extractor::FloatFeatureValue::new(
                    script_detector::Script::ScriptHiragana as u32,
                    3.0 / 5.0
                ).discrete_value()
            );
            assert_eq!(
                feature_vector[1].1,
                feature_extractor::FloatFeatureValue::new(
                    script_detector::Script::ScriptKatakana as u32,
                    2.0 / 5.0
                ).discrete_value()
            );
        }
    }
}
