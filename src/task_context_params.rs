
static LANGUAGE_NAMES: &'static [&str] = &[
    "eo", "co", "eu", "ta", "de", "mt", "ps", "te", "su", "uz", "zh-Latn", "ne",
    "nl", "sw", "sq", "hmn", "ja", "no", "mn", "so", "ko", "kk", "sl", "ig",
    "mr", "th", "zu", "ml", "hr", "bs", "lo", "sd", "cy", "hy", "uk", "pt",
    "lv", "iw", "cs", "vi", "jv", "be", "km", "mk", "tr", "fy", "am", "zh",
    "da", "sv", "fi", "ht", "af", "la", "id", "fil", "sm", "ca", "el", "ka",
    "sr", "it", "sk", "ru", "ru-Latn", "bg", "ny", "fa", "haw", "gl", "et",
    "ms", "gd", "bg-Latn", "ha", "is", "ur", "mi", "hi", "bn", "hi-Latn", "fr",
    "yi", "hu", "xh", "my", "tg", "ro", "ar", "lb", "el-Latn", "st", "ceb",
    "kn", "az", "si", "ky", "mg", "en", "gu", "es", "pl", "ja-Latn", "ga", "lt",
    "sn", "yo", "pa", "ku",
];

const LANGUAGE_IDENTIFIER_FEATURES: &str = concat!(
    "continuous-bag-of-ngrams(include_terminators=true,include_spaces=false,",
    "use_equal_weight=false,id_dim=1000,size=2);continuous-bag-of-ngrams(",
    "include_terminators=true,include_spaces=false,use_equal_weight=false,id_",
    "dim=5000,size=4);continuous-bag-of-relevant-scripts;script;continuous-bag-",
    "of-ngrams(include_terminators=true,include_spaces=false,use_equal_weight=",
    "false,id_dim=5000,size=3);continuous-bag-of-ngrams(include_terminators=",
    "true,include_spaces=false,use_equal_weight=false,id_dim=100,size=1)");

const LANGUAGE_IDENTIFIER_EMBEDDING_NAMES: &str =
    "bigrams;quadgrams;relevant-scripts;text-script;trigrams;unigrams";

const LANGUAGE_IDENTIFIER_EMBEDDING_DIMS: &str =
    "16;16;8;8;16;16";

// fn ToTaskContext(context task_context::TaskContext) {
//
// }
// void TaskContextParams::ToTaskContext(TaskContext *context) {
//   context->SetParameter("language_identifier_features",
//                         kLanguageIdentifierFeatures);
//   context->SetParameter("language_identifier_embedding_names",
//                         kLanguageIdentifierEmbeddingNames);
//   context->SetParameter("language_identifier_embedding_dims",
//                         kLanguageIdentifierEmbeddingDims);
// }

pub fn GetNumLanguages() -> i32 {
    return LANGUAGE_NAMES.len() as i32; // is it safe enough?
}
