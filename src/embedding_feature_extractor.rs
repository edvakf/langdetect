use task_context;
use sentence_features;

pub struct EmbeddingFeatureExtractor {
    // GenericEmbeddingFeatureExtractor's properties
    embedding_fml: Vec<String>,
    embedding_names: Vec<String>,
    embedding_dims: Vec<i64>,

    // EmbeddingFeatureExtractor's property
    feature_extractors: Vec<sentence_features::WholeSentenceExtractor>, // originally FeatureExtractor
}

// EmbeddingFeatureExtractor <- GenericEmbeddingFeatureExtractor
impl EmbeddingFeatureExtractor {
    // in nnet_language_identifier.cc (LanguageIdEmbeddingFeatureExtractor::ArgPrefix)
    fn arg_prefix(&self) -> String {
        "language_identifier".to_string()
    }

    // in embedding_feature_extractor.h
    fn get_param_name(&self, param_name: &str) -> String {
        self.arg_prefix() + "_" + param_name
    }

    // combination of GenericEmbeddingFeatureExtractor::Setup in embedding_feature_extractor.cc and
    // EmbeddingFeatureExtractor::Setup in embedding_feature_extractor.h
    #[allow(dead_code)]
    pub fn setup(&mut self, context: &task_context::TaskContext) {
        // GenericEmbeddingFeatureExtractor::Setup
        let feature_param = self.arg_prefix() + "_features";
        let features = context.get(feature_param, "".to_string()); // continuous-bag-of-ngrams(...);continuous-bag-of-ngrams(...);continuous-bag-of-relevant-scripts;script;continuous-bag-of-ngrams(...);continuous-bag-of-ngrams(...) etc.
        let embedding_names = context.get(self.get_param_name("embedding_names"), "".to_string()); // bigrams;quadgrams;relevant-scripts;text-script;trigrams;unigrams
        let embedding_dims = context.get(self.get_param_name("embedding_dims"), "".to_string()); // 16;16;8;8;16;16

        // self.add_strings = ...
        self.embedding_fml = features.split(";").map(|s| s.to_string()).collect();
        self.embedding_names = embedding_names.split(";").map(|s| s.to_string()).collect();

        for dim in embedding_dims.split(";") {
            self.embedding_dims.push(dim.parse::<i64>().unwrap());
        }

        // GenericEmbeddingFeatureExtractor::Setup
        // for fml in self.embedding_fml {
        //     self.feature_extractors
        //         .push(sentence_features::WholeSentenceExtractor::new(fml)); // originally FeatureExtractor
        // }
    }

    // pub fn init(context: &task_context::TaskContext) {}

    // RequestWorkspaces
    // EmbeddingSize
    // pub fn NumEmbeddings() -> i32;
}
