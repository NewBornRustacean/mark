/// MARK: Morphological Analysis with Rust for Korean.
///
///
mod corpora;
use corpora::read_corpus_from_file;

fn main() {
    let corpus =
        read_corpus_from_file("/d/data_corpus_etc/corpus_for_all_ver1.0/SXMP1902008031.json")
            .unwrap();
}
