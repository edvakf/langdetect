use std;
use feature_types;

pub type FeatureVector = Vec<(feature_types::FeatureType, FeatureValue)>;

// FeatureValue is defined by typedef in the original code
// typedef int64 Predicate;
// typedef Predicate FeatureValue;
// it may be better to make them tuple structs. not sure.
type Predicate = i64;
type FeatureValue = Predicate;

// original version uses C++ union to reinterpret (u32, f32) into i64
// here we use transmute_copy
pub struct FloatFeatureValue {
    pub id: u32,
    pub weight: f32
}

impl FloatFeatureValue {
    pub fn discrete_value(self) -> FeatureValue {
        let value: FeatureValue;
        unsafe {
            value = std::mem::transmute_copy(&self);
        }
        return value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_FloatFeatureValue() {
        assert_eq!(FloatFeatureValue{id: 0, weight: 0.0}.discrete_value(), 0);
        assert_eq!(FloatFeatureValue{id: 1, weight: 0.0}.discrete_value(), 1);
        assert_eq!(FloatFeatureValue{id: 0, weight: 1.0}.discrete_value(), 4575657221408423936);
    }

    #[test]
    fn test_FeatureVector() {
        let mut v = FeatureVector::new();
        v.push((feature_types::FeatureType::new("hoge"), 0));
        assert_eq!(v.len(), 1);
        v.clear();
        assert_eq!(v.len(), 0);
    }
}
