use std;
use feature_types;

// FeatureValue is defined by typedef in the original code
// typedef int64 Predicate;
// typedef Predicate FeatureValue;
// it may be better to make them tuple structs. not sure.
pub type Predicate = i64;
pub type FeatureValue = Predicate;

// original version uses C++ union to reinterpret (u32, f32) into i64
// here we use transmute_copy
pub struct FloatFeatureValue {
    id: u32,
    weight: f32,
}

impl FloatFeatureValue {
    pub fn new(id: u32, weight: f32) -> FloatFeatureValue {
        return FloatFeatureValue {
            id: id,
            weight: weight,
        };
    }

    pub fn discrete_value(self) -> FeatureValue {
        let value: FeatureValue;
        unsafe {
            value = std::mem::transmute_copy(&self);
        }
        return value;
    }
}

pub type FeatureVector<'a> = Vec<(&'a feature_types::FeatureType, FeatureValue)>;

// originally GenericFeatureFunction's constexpr `kNone`
pub const NONE: FeatureValue = -1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_FloatFeatureValue() {
        assert_eq!(FloatFeatureValue { id: 0, weight: 0.0 }.discrete_value(), 0);
        assert_eq!(FloatFeatureValue { id: 1, weight: 0.0 }.discrete_value(), 1);
        assert_eq!(
            FloatFeatureValue { id: 0, weight: 1.0 }.discrete_value(),
            4575657221408423936
        );
    }
}
