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

pub type FeatureVector<'a, T: feature_types::FeatureType> = Vec<(&'a T, FeatureValue)>;

pub trait GenericFeatureFunction<T: feature_types::FeatureType> {
    // fn setup(self);
    fn init(&mut self);
}

// originally GenericFeatureFunction's constexpr `kNone`
const NONE: FeatureValue = -1;

pub trait FeatureFunction<T: feature_types::FeatureType> {
    type Obj;

    fn evaluate(&self, obj: Self::Obj) -> FeatureVector<T>; // TODO: workspace and args

    // fn compute(&self, obj: Self::Obj) -> FeatureValue {
    //     return NONE;
    // }
}

// type WholeSentenceFeature = FeatureFunction<Sentence>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_FloatFeatureValue() {
        assert_eq!(FloatFeatureValue{id: 0, weight: 0.0}.discrete_value(), 0);
        assert_eq!(FloatFeatureValue{id: 1, weight: 0.0}.discrete_value(), 1);
        assert_eq!(FloatFeatureValue{id: 0, weight: 1.0}.discrete_value(), 4575657221408423936);
    }
}
