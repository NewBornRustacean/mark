use std::char::from_u32;
use std::collections::HashMap;

const HANGUL_START: u32 = 44032; // unicode value of '가'
const HANGUL_END: u32 = 55203;
const NUM_CHO: u32 = 19; // 초성의 개수
const NUM_JUNG: u32 = 21; // 중성의 개수
const NUM_JONG: u32 = 28; // 종성의 개수, "없음" 포함

const CHO_SUNG: [char; NUM_CHO as usize] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];
const JUNG_SUNG: [char; NUM_JUNG as usize] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];

const JONG_SUNG: [char; NUM_JONG as usize] = [
    // 종성이 없는 경우 empty literal => JONG_SUNG[0]= '\0'
    '\0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ',
    'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

/// Def:
///     return true if the input syllable is in unicode scope of valid Hangul.
/// Note:
///     this function is valid only for the modern Korean chars.
///
fn is_hangul(syllable: char) -> bool {
    let syllable_unicode = syllable as u32;

    if syllable_unicode > HANGUL_END || syllable_unicode < HANGUL_START {
        return false;
    } else {
        return true;
    }
}

/// Def:
///     get unicode value given cho, jung, jong index.
/// Note:
///     초성의 인덱스는 588(=중성의 개수*종성의 개수) 글자마다 바뀜.
fn get_char_from_indices(cho_idx: u32, jung_idx: u32, jong_idx: u32) -> char {
    let res: u32 =
        ((cho_idx * NUM_JUNG * NUM_JONG) + (jung_idx * NUM_JONG) + jong_idx) + HANGUL_START;
    return from_u32(res).unwrap();
}

/// Def:
///     get indices of given syllable(="음절"; a single char of Hangul)
/// Note:
///     초성 = (음절의 유니코드 - 오프셋) / (중성개수*종성개수)
///     중성 = (음절의 유니코드 - 오프셋 - (초성*중성개수*종성개수)) / (종성개수)
///     종성 = (음절의 유니코드 - 오프셋 - (초성*중성개수*종성개수) - (중성*종성개수))
fn get_indices_from_syllable(syllable: char) -> (u32, u32, u32) {
    let syllable_uni: u32 = syllable as u32;

    let cho_idx = (syllable_uni - HANGUL_START) / (NUM_JUNG * NUM_JONG);
    let jung_idx = (syllable_uni - HANGUL_START - (cho_idx * NUM_JUNG * NUM_JONG)) / NUM_JONG;
    let jong_idx =
        syllable_uni - HANGUL_START - (cho_idx * NUM_JUNG * NUM_JONG) - (jung_idx * NUM_JONG);

    return (cho_idx, jung_idx, jong_idx);
}

#[cfg(test)]
mod test_korean_strings {
    use super::*;

    #[test]
    fn test_is_hangul() {
        // positive case
        let mut test_char = '헿';
        assert_eq!(is_hangul(test_char), true);

        // negative case: english
        test_char = 'z';
        assert_eq!(is_hangul(test_char), false);

        // negative case: digit
        test_char = '1';
        assert_eq!(is_hangul(test_char), false);

        // negative case: special char
        test_char = '!';
        assert_eq!(is_hangul(test_char), false);
    }
    #[test]
    fn test_get_indices_from_unicode() {
        // unicodef for '가', which is "no jong-sung" case.
        let mut test_char = '가';
        let mut indices = get_indices_from_syllable(test_char);
        assert_eq!('ㄱ', CHO_SUNG[indices.0 as usize]);
        assert_eq!('ㅏ', JUNG_SUNG[indices.1 as usize]);
        assert_eq!('\0', JONG_SUNG[indices.2 as usize]);
        assert_eq!(
            get_char_from_indices(indices.0, indices.1, indices.2),
            test_char
        );

        // unicodef for '안', which is "cho-jung-jong" case.
        test_char = '안';
        indices = get_indices_from_syllable(test_char);
        assert_eq!('ㅇ', CHO_SUNG[indices.0 as usize]);
        assert_eq!('ㅏ', JUNG_SUNG[indices.1 as usize]);
        assert_eq!('ㄴ', JONG_SUNG[indices.2 as usize]);
        assert_eq!(
            get_char_from_indices(indices.0, indices.1, indices.2),
            test_char
        );

        // unicodef for '밝', which is "complex jong-sung" case.
        test_char = '밝';
        indices = get_indices_from_syllable(test_char);
        assert_eq!('ㅂ', CHO_SUNG[indices.0 as usize]);
        assert_eq!('ㅏ', JUNG_SUNG[indices.1 as usize]);
        assert_eq!('ㄺ', JONG_SUNG[indices.2 as usize]);
        assert_eq!(
            get_char_from_indices(indices.0, indices.1, indices.2),
            test_char
        );

        // unicodef for '놔', which is "complex jung-sung" case.
        test_char = '놔';
        indices = get_indices_from_syllable(test_char);
        assert_eq!('ㄴ', CHO_SUNG[indices.0 as usize]);
        assert_eq!('ㅘ', JUNG_SUNG[indices.1 as usize]);
        assert_eq!('\0', JONG_SUNG[indices.2 as usize]);
        assert_eq!(
            get_char_from_indices(indices.0, indices.1, indices.2),
            test_char
        );
    }
}
