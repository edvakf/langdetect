use feature_extractor;
use language_identifier_features;

pub struct WholeSentenceExtractor {
    feature_type: language_identifier_features::NumericFeatureType,
    function: Box<WholeSentenceFeature>,
}

// WholeSentenceExtractor <- FeatureExtractor <- GenericFeatureExtractor
impl WholeSentenceExtractor {
    // replacement of GenericFeatureExtractor::Parse, Setup and Init
    // the Parse method parses the FML into functions, but we skip that
    pub fn new(source: String) -> WholeSentenceExtractor {
        let f: Box<WholeSentenceFeature>;
        let ft: language_identifier_features::NumericFeatureType;
        if source == "continuous-bag-of-ngrams(include_terminators=true,include_spaces=false,use_equal_weight=false,id_dim=1000,size=2)" {
            ft = language_identifier_features::NumericFeatureType::new(
                "continuous-bag-of-ngrams",
                1000, // id_dim
            );
            f = Box::new(language_identifier_features::ContinuousBagOfNgramsFunction::new(true, false, false, 1000, 2, ft));
        } else {
            panic!("unparsable FML");
        }
        return WholeSentenceExtractor {
            feature_type: ft,
            function: f,
        };
    }

    // FeatureExtractor::ExtractFeatures in feature_extractor.h
    fn extract_features(&self, sentence: String) -> feature_extractor::FeatureVector {
        self.function.evaluate(sentence)
    }
}

// WholeSentenceFeature <- FeatureFunction <- GenericFeatureFunction
pub trait WholeSentenceFeature {
    fn feature_type(&self) -> &language_identifier_features::NumericFeatureType;

    // FeatureFunction::Evaluate's default implementation (just calls Capture) in feature_extractor.h
    fn evaluate(&self, sentence: String) -> feature_extractor::FeatureVector {
        let mut result = feature_extractor::FeatureVector::new();
        let value = self.capture(sentence, &result);
        if value != feature_extractor::NONE {
            result.push((self.feature_type(), value));
        }
        result
    }

    // FeatureFunction::Capture's default implementation in feature_extractor.h
    fn capture(
        &self,
        sentence: String,
        feature_vector: &feature_extractor::FeatureVector,
    ) -> feature_extractor::FeatureValue {
        feature_extractor::NONE
    }
}
