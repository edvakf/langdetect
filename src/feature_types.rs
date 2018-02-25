
pub struct FeatureType {
    pub name: String,
    pub base: i64,
    pub is_continuous: bool
}

impl FeatureType {
    pub fn new(name: &str) -> Self {
        return FeatureType{
            name: name.to_string(),
            base: 0,
            is_continuous: name.contains("continuous")
        };
    }
}
