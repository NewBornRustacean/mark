/// MARK: Morphological Analysis with Rust for Korean.
///
///
mod corpora;
mod hangul;
mod trie;

fn main() {
    let niadict = corpora::read_nia_morphs("resources/nia_json.json").unwrap();
    println!("{:?}", niadict.len());
}
