use std::char::from_u32;

/// 상수 정의: 유니코드 값 계산을 위해 usize type 사용.
const HANGUL_START: usize = 44032; // unicode value of '가'
const HANGUL_END: usize = 55203;
const NUM_INITIAL_CONSONANT: usize = 19; // 초성의 개수
const NUM_MID_VOWEL: usize = 21; // 중성의 개수
const NUM_FINAL_CONSONANT: usize = 28; // 종성의 개수, "없음" 포함

const CHO_SUNG: [char; NUM_INITIAL_CONSONANT] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];
const JUNG_SUNG: [char; NUM_MID_VOWEL] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];

const JONG_SUNG: [char; NUM_FINAL_CONSONANT] = [
    // 종성이 없는 경우 empty literal => JONG_SUNG[0]= '\0'
    '\0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ',
    'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

/// 품사(POS; Part of Speech) 태그 정의
#[derive(Debug)]
pub enum PosTag { // 세종계획 품사태그
    SP, //쉼표
    SF, //마침표
    SU, //단위기호
    SY, //기타기호
    SD, //이음표
    EP, //종결어미
    XP, //접두사
    JCS, //주격조사
    JXC, //통용보조사
    NCN, //비서술성 명사
    PVG, //일반동사
    PAD, //지시형용사
    MAG, //일반부사
    MMD, //지시관형사
    II, //감탄사
    NBU, //단위성 의존 명사
    NNC, //양수사
    NPP, //인칭대명사
    ECC, //대등적 연결어미
    EF, //종결어미
    ETN, //명사형어미
    XSN, //명사파생접미사
    F, //외국어
}

/// Def:
///     get unicode value given cho, jung, jong index.
/// Note:
///     초성의 인덱스는 588(=중성의 개수*종성의 개수) 글자마다 바뀜.
fn get_char_from_indices(cho_idx: usize, jung_idx: usize, jong_idx: usize) -> char {
    let res: usize = ((cho_idx * NUM_MID_VOWEL * NUM_FINAL_CONSONANT)
        + (jung_idx * NUM_FINAL_CONSONANT)
        + jong_idx)
        + HANGUL_START;
    return from_u32(res as u32).unwrap();
}

/// Def:
///     get indices of given syllable(="음절"; a single char of Hangul)
/// Note:
///     초성 = (음절의 유니코드 - 오프셋) / (중성개수*종성개수)
///     중성 = ((음절의 유니코드 - 오프셋) / 종성개수) % 중성개수
///     종성 = (음절의 유니코드 - 오프셋) % 종성개수
fn get_indices_from_syllable(syllable: char) -> (usize, usize, usize) {
    let syllable_uni = syllable as usize - HANGUL_START;

    let cho_idx = syllable_uni / (NUM_MID_VOWEL * NUM_FINAL_CONSONANT);
    let jung_idx = (syllable_uni / NUM_FINAL_CONSONANT) % NUM_MID_VOWEL;
    let jong_idx = syllable_uni % NUM_FINAL_CONSONANT;

    return (cho_idx, jung_idx, jong_idx);
}

/// Def:
///     return true if the input syllable is in unicode scope of valid Hangul.
/// Note:
///     this function is valid only for the modern Korean chars.
///
pub fn is_hangul(syllable: char) -> bool {
    let syllable_unicode = syllable as usize;

    if syllable_unicode > HANGUL_END || syllable_unicode < HANGUL_START {
        return false;
    } else {
        return true;
    }
}

/// Def:
///     한글 한 음절을 초성, 중성, 종성 문자로 분리해서 반환
/// Note:
///     종성이 없는 경우 종성 테이블의 0번 원소가 반환됨:= '\0'
pub fn split_syllable(syllable: char) -> (char, char, char) {
    if is_hangul(syllable) == false {
        panic!("given syllable is NOT a Hangul.")
    }

    let (cho_idx, jung_idx, jong_idx) = get_indices_from_syllable(syllable);
    return (CHO_SUNG[cho_idx], JUNG_SUNG[jung_idx], JONG_SUNG[jong_idx]);
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
        assert_eq!('ㄱ', CHO_SUNG[indices.0]);
        assert_eq!('ㅏ', JUNG_SUNG[indices.1]);
        assert_eq!('\0', JONG_SUNG[indices.2]);
        assert_eq!(
            get_char_from_indices(indices.0, indices.1, indices.2),
            test_char
        );

        // unicodef for '안', which is "cho-jung-jong" case.
        test_char = '안';
        indices = get_indices_from_syllable(test_char);
        assert_eq!('ㅇ', CHO_SUNG[indices.0]);
        assert_eq!('ㅏ', JUNG_SUNG[indices.1]);
        assert_eq!('ㄴ', JONG_SUNG[indices.2]);
        assert_eq!(
            get_char_from_indices(indices.0, indices.1, indices.2),
            test_char
        );

        // unicodef for '밝', which is "complex jong-sung" case.
        test_char = '밝';
        indices = get_indices_from_syllable(test_char);
        assert_eq!('ㅂ', CHO_SUNG[indices.0]);
        assert_eq!('ㅏ', JUNG_SUNG[indices.1]);
        assert_eq!('ㄺ', JONG_SUNG[indices.2]);
        assert_eq!(
            get_char_from_indices(indices.0, indices.1, indices.2),
            test_char
        );

        // unicodef for '놔', which is "complex jung-sung" case.
        test_char = '놔';
        indices = get_indices_from_syllable(test_char);
        assert_eq!('ㄴ', CHO_SUNG[indices.0]);
        assert_eq!('ㅘ', JUNG_SUNG[indices.1]);
        assert_eq!('\0', JONG_SUNG[indices.2]);
        assert_eq!(
            get_char_from_indices(indices.0, indices.1, indices.2),
            test_char
        );
    }

    #[test]
    fn test_split_syllable() {
        // '가' => 'ㄱ', 'ㅏ', '\0'
        let mut test_char = '가';
        let (initial_consonant, mid_vowel, final_consonant) = split_syllable(test_char);
        assert_eq!(initial_consonant, 'ㄱ');
        assert_eq!(mid_vowel, 'ㅏ');
        assert_eq!(final_consonant, '\0');

        // '헿' => 'ㅎ', 'ㅔ', 'ㅎ'
        test_char = '헿';
        let (initial_consonant, mid_vowel, final_consonant) = split_syllable(test_char);
        assert_eq!(initial_consonant, 'ㅎ');
        assert_eq!(mid_vowel, 'ㅔ');
        assert_eq!(final_consonant, 'ㅎ');

        // '왕' => 'ㅇ', 'ㅘ', 'ㅇ'
        test_char = '왕';
        let (initial_consonant, mid_vowel, final_consonant) = split_syllable(test_char);
        assert_eq!(initial_consonant, 'ㅇ');
        assert_eq!(mid_vowel, 'ㅘ');
        assert_eq!(final_consonant, 'ㅇ');

        // '뚫' => 'ㄸ', 'ㅜ', 'ㅀ'
        test_char = '뚫';
        let (initial_consonant, mid_vowel, final_consonant) = split_syllable(test_char);
        assert_eq!(initial_consonant, 'ㄸ');
        assert_eq!(mid_vowel, 'ㅜ');
        assert_eq!(final_consonant, 'ㅀ');
    }
    #[test]
    #[should_panic]
    fn test_split_syllable_goes_panic() {
        // digit case
        let mut test_char = '1';
        split_syllable(test_char);

        // english case
        test_char = 'c';
        split_syllable(test_char);

        // special char case
        test_char = '#';
        split_syllable(test_char);
    }
}
