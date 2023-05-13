use std::char::from_u32;

/// 상수 정의: 유니코드 값 계산을 위해 usize type 사용.
const HANGUL_START: usize = 44032; // unicode value of '가'
const HANGUL_END: usize = 55203;
const NUM_INITIAL_CONSONANT: usize = 19; // 초성의 개수
const NUM_VOWEL_CONSONANT: usize = 21; // 중성의 개수
const NUM_FINAL_CONSONANT: usize = 28; // 종성의 개수, "없음" 포함

const CHO_SUNG: [char; NUM_INITIAL_CONSONANT] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];
const JUNG_SUNG: [char; NUM_VOWEL_CONSONANT] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];

const JONG_SUNG: [char; NUM_FINAL_CONSONANT] = [
    // 종성이 없는 경우 empty literal => JONG_SUNG[0]= '\0'
    '\0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ',
    'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

/// 품사(POS; Part of Speech) 태그 정의
/// "모두의 말뭉치"에서 정의한 품사 태그를 그대로 사용.
pub enum PosTag {
    NNG, //일반명사
    NNP, //고유명사
    NNB, //의존명사
    NP,  //대명사
    NR,  //수사
    VV,  //동사
    VA,  //형용사
    VX,  //보조용언
    VCP, //긍정지정사
    VCN, //부정지정사
    MMA, //성상 관형사
    MMD, //지시 관형사
    MMN, //수 관형사
    MAG, //일반부사
    MAJ, //접속부사
    IC,  //감탄사
    JKS, //주격조사
    JKC, //보격조사
    JKG, //관형격조사
    JKO, //목적격조사
    JKB, //부사격조사
    JKV, //호격조사
    JKQ, //인용격조사
    JX,  //보조사
    JC,  //접속조사
    EP,  //선어말어미
    EF,  //종결어미
    EC,  //연결어미
    ETN, //명사형전성어미
    ETM, //관형형전성어미
    XPN, //체언접두사
    XSN, //명사파생접미사
    XSV, //동사파생접미사
    XSA, //형용사파생접미사
    XR,  //어근
    SF,  //마침표, 물음표, 느낌표
    SP,  //쉼표, 가운뎃점, 콜론, 빗금
    SS,  //따옴표, 괄호표, 줄표
    SE,  //줄임표
    SO,  //붙임표(물결)
    SW,  //기타 기호
    SL,  //외국어
    SH,  //한자
    SN,  //숫자
    NA,  //분석불능범주
    NF,  //명사추정범주
    NV,  //용언추정범주
}

/// Def:
///     return true if the input syllable is in unicode scope of valid Hangul.
/// Note:
///     this function is valid only for the modern Korean chars.
///
fn is_hangul(syllable: char) -> bool {
    let syllable_unicode = syllable as usize;

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
fn get_char_from_indices(cho_idx: usize, jung_idx: usize, jong_idx: usize) -> char {
    let res: usize = ((cho_idx * NUM_VOWEL_CONSONANT * NUM_FINAL_CONSONANT)
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
    if is_hangul(syllable) == false {
        panic!("given syllable is NOT a Hangul.")
    }
    let syllable_uni = syllable as usize - HANGUL_START;

    let cho_idx = syllable_uni / (NUM_VOWEL_CONSONANT * NUM_FINAL_CONSONANT);
    let jung_idx = (syllable_uni / NUM_FINAL_CONSONANT) % NUM_VOWEL_CONSONANT;
    let jong_idx = syllable_uni % NUM_FINAL_CONSONANT;

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
    #[should_panic]
    fn test_get_indices_from_unicode_goes_panic() {
        // digit case
        let mut test_char = '1';
        get_indices_from_syllable(test_char);

        // english case
        test_char = 'c';
        get_indices_from_syllable(test_char);

        // special char case
        test_char = '#';
        get_indices_from_syllable(test_char);
    }
}
