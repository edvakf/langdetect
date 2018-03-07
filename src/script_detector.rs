// Unicode scripts we care about.  To get compact and fast code, we detect only
// a few Unicode scripts that offer a strong indication about the language of
// the text (e.g., Hiragana -> Japanese).
#[derive(PartialEq, Debug)]
enum Script {
    // Special value to indicate internal errors in the script detection code.
    ScriptError,

    // Special values for all Unicode scripts that we do not detect.  One special
    // value for Unicode characters of 1, 2, 3, respectively 4 bytes (as we
    // already have that information, we use it).  ScriptOtherUtf8OneByte means
    // ~Latin and ScriptOtherUtf8FourBytes means ~Han.
    ScriptOtherUtf8OneByte,
    ScriptOtherUtf8TwoBytes,
    ScriptOtherUtf8ThreeBytes,
    ScriptOtherUtf8FourBytes,

    ScriptGreek,
    ScriptCyrillic,
    ScriptHebrew,
    ScriptArabic,
    ScriptHangulJamo, // Used primarily for Korean.
    ScriptHiragana,   // Used primarily for Japanese.
    ScriptKatakana,   // Used primarily for Japanese.

    // Add new scripts here.

    // Do not add any script after kNumRelevantScripts.  This value indicates the
    // number of elements in this enum Script (except this value) such that we can
    // easily iterate over the scripts.
    NumRelevantScripts,
}

fn in_range<T: Ord>(value: T, low: T, hi: T) -> bool {
    (value >= low) && (value <= hi)
}

fn get_script(c: char) -> Script {
    match c.len_utf8() {
        1 => {
            return Script::ScriptOtherUtf8OneByte;
        }
        2 => {
            static GREEK_START: u32 = 0x370;
            static CYRILLIC_START: u32 = 0x400;
            static CYRILLIC_END: u32 = 0x4FF;
            static HEBREW_START: u32 = 0x590;
            static ARABIC_START: u32 = 0x600;
            static ARABIC_END: u32 = 0x6FF;

            let codepoint = c as u32;
            if codepoint > CYRILLIC_END {
                if codepoint >= ARABIC_START {
                    if codepoint <= ARABIC_END {
                        return Script::ScriptArabic;
                    }
                } else {
                    // At this point, codepoint < kArabicStart = kHebrewEnd + 1, so
                    // codepoint <= kHebrewEnd.
                    if codepoint >= HEBREW_START {
                        return Script::ScriptHebrew;
                    }
                }
            } else {
                if codepoint >= CYRILLIC_START {
                    return Script::ScriptCyrillic;
                } else {
                    // At this point, codepoint < CYRILLIC_START = kGreekEnd + 1, so
                    // codepoint <= kGreekEnd.
                    if codepoint >= GREEK_START {
                        return Script::ScriptGreek;
                    }
                }
            }
            return Script::ScriptOtherUtf8TwoBytes;
        }
        3 => {
            // 3-byte UTF8 characters have 16 bits of information.  unsigned int has
            // at least 16 bits.
            static HANGUL_JAMO_START: u32 = 0x1100;
            static HANGUL_JAMO_END: u32 = 0x11FF;
            static HIRAGANA_START: u32 = 0x3041;
            static HIRAGANA_END: u32 = 0x309F;

            // Commented out (unsued in the code): kKatakanaStart = 0x30A0;
            static KATAKANA_END: u32 = 0x30FF;
            let codepoint = c as u32;
            if codepoint > HIRAGANA_END {
                // On this branch, codepoint > kHiraganaEnd = kKatakanaStart - 1, so
                // codepoint >= kKatakanaStart.
                if codepoint <= KATAKANA_END {
                    return Script::ScriptKatakana;
                }
            } else {
                if codepoint >= HIRAGANA_START {
                    return Script::ScriptHiragana;
                } else {
                    if in_range(codepoint, HANGUL_JAMO_START, HANGUL_JAMO_END) {
                        return Script::ScriptHangulJamo;
                    }
                }
            }
            return Script::ScriptOtherUtf8ThreeBytes;
        }
        4 => {
            return Script::ScriptOtherUtf8FourBytes;
        }
        _ => {
            return Script::ScriptError;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::get_script;
    use super::Script;

    #[test]
    fn test_get_script() {
        assert_eq!(get_script('a'), Script::ScriptOtherUtf8OneByte);
        assert_eq!(get_script('Σ'), Script::ScriptGreek);
        assert_eq!(get_script('Щ'), Script::ScriptCyrillic);
        assert_eq!(get_script('א'), Script::ScriptHebrew);
        assert_eq!(get_script('ج'), Script::ScriptArabic);
        assert_eq!(get_script('ᄒ'), Script::ScriptHangulJamo);
        assert_eq!(get_script('あ'), Script::ScriptHiragana);
        assert_eq!(get_script('ア'), Script::ScriptKatakana);
        assert_eq!(get_script('漢'), Script::ScriptOtherUtf8ThreeBytes);
        assert_eq!(get_script('🍺'), Script::ScriptOtherUtf8FourBytes);
    }

    #[test]
    fn test_num_relevant_scripts() {
        assert_eq!(Script::NumRelevantScripts as i32, 12);
    }
}
