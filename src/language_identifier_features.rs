use std::collections::HashMap;
use feature_extractor;
use utils;
use sentence_features;
use feature_types;

// NumericFeatureType and it's base class FeatureType combined
#[derive(Copy, Clone)]
pub struct NumericFeatureType {
    // FeatureType's properties
    name: &'static str,
    // base: feature_extractor::Predicate,
    is_continuous: bool,

    // NumericFeatureType's property
    size: feature_extractor::FeatureValue,
}

impl NumericFeatureType {
    pub fn new(name: &'static str, size: feature_extractor::FeatureValue) -> Self {
        let is_continuous = name.contains("continuous");

        return NumericFeatureType {
            name: name,
            // base: 0,
            is_continuous: is_continuous,
            size: size,
        };
    }
}

impl feature_types::FeatureType for NumericFeatureType {
    fn name(&self) -> &'static str {
        &self.name
    }

    fn is_continuous(&self) -> bool {
        self.is_continuous
    }
}

// please refer to cld3's language_identifier_features.h
pub struct ContinuousBagOfNgramsFunction {
    // If 'true', then splits the text based on spaces to get tokens, adds "^" to
    // the beginning of each token, and adds "$" to the end of each token.
    include_terminators: bool,

    // If 'true', then includes char ngrams containing spaces.
    include_spaces: bool,

    // If 'true', then weighs each unique ngram by 1.0 / (number of unique ngrams
    // in the input). Otherwise, weighs each unique ngram by (ngram count) /
    // (total number of ngrams).
    use_equal_ngram_weight: bool,

    // The integer id of each char ngram is computed as follows:
    // Hash32WithDefaultSeed(char_ngram) % ngram_id_dimension_.
    ngram_id_dimension: i32,

    // Only ngrams of size ngram_size_ will be extracted.
    ngram_size: usize,

    feature_type: NumericFeatureType,
}

impl ContinuousBagOfNgramsFunction {
    pub fn new(
        include_terminators: bool,
        include_spaces: bool,
        use_equal_ngram_weight: bool,
        ngram_id_dimension: i32,
        ngram_size: usize,
        feature_type: NumericFeatureType,
    ) -> ContinuousBagOfNgramsFunction {
        return ContinuousBagOfNgramsFunction {
            include_terminators: include_terminators,
            include_spaces: include_spaces,
            use_equal_ngram_weight: use_equal_ngram_weight,
            ngram_id_dimension: ngram_id_dimension,
            ngram_size: ngram_size,
            feature_type: feature_type,
        };
    }
}

impl sentence_features::WholeSentenceFeature for ContinuousBagOfNgramsFunction {
    fn feature_type(&self) -> &NumericFeatureType {
        &self.feature_type
    }

    #[allow(dead_code)]
    fn evaluate(&self, sentence: String) -> feature_extractor::FeatureVector {
        let mut s: String;
        // let mut chars;

        let mut mark: Vec<(usize, usize, bool)> = vec![]; // startPos, endPos, isSpace

        // Include terminators for each token. Tokens are discovered by splitting the
        // text on spaces.
        if self.include_terminators {
            s = String::new();
            s.push('^');
            mark.push((0, 1, false));

            let mut pos: usize = 1;
            for c in sentence.chars() {
                if c == ' ' {
                    s.push('$');
                    mark.push((pos, pos + 1, false));
                    s.push(' ');
                    mark.push((pos + 1, pos + 2, true));
                    s.push('^');
                    mark.push((pos + 2, pos + 3, false));
                    pos += 3;
                } else {
                    s.push(c);
                    mark.push((pos, pos + c.len_utf8(), false));
                    pos += c.len_utf8();
                }
            }
            s.push('$');
            mark.push((pos, pos + 1, false));
        } else {
            s = sentence
                .char_indices()
                .map(|(i, c)| {
                    mark.push((i, i + c.len_utf8(), c == ' '));
                    c
                })
                .collect();
        }
        // println!("{}", s);
        // println!("{:?}", mark);
        // println!("{}", chars.collect::<String>());

        // Find the char ngram counts.
        let mut char_ngram_counts: HashMap<String, i32> = HashMap::new();
        let mut count_sum: i32 = 0;

        for i in 0..(mark.len() - self.ngram_size) {
            if !self.include_spaces {
                let mut has_space = false;
                for j in 0..(self.ngram_size + 1) {
                    has_space = has_space || mark[i + j].2;
                }
                // println!("{}", has_space);
                if has_space {
                    continue;
                }
            }
            let key = String::from(&s[(mark[i].0)..(mark[i + self.ngram_size].1)]);
            // println!("{}", key);
            *char_ngram_counts.entry(key).or_insert(0) += 1;
            count_sum += 1;
        }

        // Populate the feature vector.
        let equal_weight = 1.0 / (char_ngram_counts.len() as f32);
        let norm = count_sum as f32;
        let mut feature_vector = feature_extractor::FeatureVector::new();

        for (key, count) in &char_ngram_counts {
            let feature_value = feature_extractor::FloatFeatureValue {
                id: utils::has32_with_default_seed(key),
                weight: if self.use_equal_ngram_weight {
                    equal_weight
                } else {
                    *count as f32 / norm
                },
            }.discrete_value();
            // println!("{}", key);
            // println!("{}", utils::has32_with_default_seed(key));
            // println!("{}", feature_value);
            feature_vector.push((
                self.feature_type() as &feature_types::FeatureType,
                feature_value,
            ));
        }

        return feature_vector;
    }
}

#[cfg(test)]
mod tests {
    use super::ContinuousBagOfNgramsFunction;
    use super::NumericFeatureType;
    // use super::feature_extractor::FeatureFunction;
    use super::feature_types::FeatureType;
    // use super::feature_extractor::FeatureValue;
    use super::sentence_features::WholeSentenceFeature;

    #[test]
    fn test_evaluate() {
        let f = &ContinuousBagOfNgramsFunction {
            include_terminators: true,
            include_spaces: false,
            use_equal_ngram_weight: true,
            ngram_id_dimension: 100,
            ngram_size: 2,
            feature_type: NumericFeatureType::new("continuous-bag-of-ngrams", 10),
        };
        let v = f.evaluate("hoge fuga".to_string());

        assert_eq!(v.len(), 8);
        assert_eq!(v[0].0.name(), "continuous-bag-of-ngrams");
    }

    #[test]
    fn test_numeric_feature_type() {
        let ft = NumericFeatureType::new("continuous-bag-of-ngrams", 10);
        assert!(ft.is_continuous());
        assert_eq!(ft.size, 10);
    }
}
