// originally FeatureType which is the base class of NumericFeatureType etc.
pub trait FeatureType {
    fn name(&self) -> &'static str;

    fn is_continuous(&self) -> bool;
}
