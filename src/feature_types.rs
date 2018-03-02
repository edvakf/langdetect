use feature_extractor;

// originally FeatureType which is the base class of NumericFeatureType etc.
pub struct FeatureTypeBase {
    name: String,
    base: feature_extractor::Predicate,
    is_continuous: bool
}

impl FeatureTypeBase {
    pub fn new(name: String) -> Self {
        let is_continuous = name.contains("continuous");

        return FeatureTypeBase{
            name: name,
            base: 0,
            is_continuous: is_continuous
        };
    }
}

pub trait FeatureType {
    // Should be overriiden by an impl of FeatureType
    fn get_feature_type_base(&self) -> &FeatureTypeBase;

    // Should be overriiden by an impl of FeatureType
    fn get_mutable_feature_type_base<'a>(&'a mut self) -> &'a mut FeatureTypeBase;

    fn name(&self) -> &String {
        &self.get_feature_type_base().name
    }

    fn base(&self) -> feature_extractor::Predicate {
        self.get_feature_type_base().base
    }

    fn set_base(&mut self, base: feature_extractor::Predicate) {
        self.get_mutable_feature_type_base().base = base;
    }

    fn is_continuous(&self) -> bool {
        self.get_feature_type_base().is_continuous
    }
}
