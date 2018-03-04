use feature_extractor;
use feature_types;

pub trait WholeSentenceFeature {
    fn set_feature_type(&mut self, feature_type: feature_types::FeatureType);

    fn feature_type(&self) -> &feature_types::FeatureType;

    // GenericFeatureFunction::Setup's default implementation in feature_extractor.h
    fn setup(&mut self) {}

    // GenericFeatureFunction::Init's default implementation in feature_extractor.h
    fn init(&mut self) {}

    // FeatureFunction::Evaluate's default implementation (just calls Capture) in feature_extractor.h
    fn evaluate(&self, sentence: String) -> feature_extractor::FeatureVector {
        let result = feature_extractor::FeatureVector::new();
        let value = self.capture(sentence, result);
        if value != feature_extractor::NONE {
            result.push((self.feature_type(), value));
        }
        result
    }

    // FeatureFunction::Capture's default implementation in feature_extractor.h
    fn capture(&self, sentence: String, feature_vector: &feature_extractor::FeatureVector) -> feature_extractor::FeatureValue {
        feature_extractor::NONE
    }
}
