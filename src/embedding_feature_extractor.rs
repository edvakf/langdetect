
pub struct EmbeddingFeatureExtractor<T> {
    feature_extractors: FeatureExtractor
}

trait EmbeddingFeatureExtractor {
    pub fn ArgPrefix() -> String;

    pub fn Setup(self, context: &TaskContext) {
        let feature_param = self.ArgPrefix() + "_features".to_string();
        let embedding_names = context.Get(GetParamName("embedding_names"), "");
        let embedding_dims = context.Get(GetParamName("embedding_dims"), "");
        let add_varlen_strings = context.Get(GetParamName("add_varlen_strings"), false);

        // TODO: 
    }

    pub fn Init(context: &TaskContext);
    // RequestWorkspaces
    // EmbeddingSize
    pub fn NumEmbeddings() -> i32;

    fn GetParamName(param_name: String) {
        return self.ArgPrefix() + "_".to_string() + param_name;
    }
}
