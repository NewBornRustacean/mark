/// MARK: Morphological Analysis with Rust for Korean.
///
///
mod corpora;
use corpora::{make_morphemes_unique, read_uniq_morphs, save_morphemes_to_json};
mod hangul;
mod trie;

fn main() {
    // a number of documents in SXMP: 423
    let uniq_morphs =
        make_morphemes_unique("D:\\data_corpus_etc\\corpus_for_all_ver1.0\\NXMP1902008040.json")
            .unwrap();
    // make_morphemes_unique("D:\\data_corpus_etc\\corpus_for_all_ver1.0\\SXMP1902008031.json")
    //     .unwrap();

    println!("{:?}", uniq_morphs[0]);
    save_morphemes_to_json(&uniq_morphs, "resources/uniq_morphs_NXMP.json").unwrap();
    let uniq_morphs = read_uniq_morphs("resources/uniq_morphs_NXMP.json").unwrap();
    println!("{:?}\n {:?}", uniq_morphs[0], uniq_morphs.len()); // #of unique morphemes in SXMP = 32402
}
