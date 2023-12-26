//! trie structure and its functions
//!
//!
use std::collections::HashMap;

use crate::hangul::{is_hangul, split_syllable, PosTag};

#[derive(Default, Debug)]
struct TrieNode {
    is_end: bool,
    children: HashMap<char, TrieNode>,
    tag: Option<PosTag>, // this field has a value if and only if is_end:=True
}

impl TrieNode {
    pub fn new(is_end: bool, tag: Option<PosTag>) -> Self {
        TrieNode {
            is_end: is_end,
            children: HashMap::default(),
            tag: tag,
        }
    }

    pub fn default() -> Self {
        TrieNode {
            is_end: false,
            children: HashMap::default(),
            tag: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
    total: u32, // the number of nodes in this trie(>= the number of morphemes in dictionary).
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
            total: 0,
        }
    }

    pub fn insert(&mut self, word: &str, tag: Option<PosTag>) {
        let mut current_node: &mut TrieNode = &mut self.root;

        for single_char in word.chars() {
            // iterate over word to deal with each of syllables
            match is_hangul(single_char) {
                true => {
                    // 한글은 초성, 중성, 종성 나눠서 insert
                    let (initial_consonant, mid_vowel, final_consonant) =
                        split_syllable(single_char);

                    if !current_node.children.contains_key(&initial_consonant) {
                        self.total += 1;
                    }
                    current_node = current_node.children.entry(initial_consonant).or_default();

                    if !current_node.children.contains_key(&mid_vowel) {
                        self.total += 1;
                    }
                    current_node = current_node.children.entry(mid_vowel).or_default();

                    if final_consonant.ne(&'\0') {
                        if !current_node.children.contains_key(&final_consonant) {
                            self.total += 1;
                        }
                        current_node = current_node.children.entry(final_consonant).or_default();
                    }
                }
                false => {
                    // 한글이 아닌 문자는 그냥 insert
                    current_node = current_node.children.entry(single_char).or_default();
                    self.total += 1;
                }
            }
        }
        current_node.is_end = true;

        if tag.is_none() == false {
            current_node.tag = tag;
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        // NOT implemented
        false
    }
}

#[cfg(test)]
mod test_trie {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();

        trie.insert("역삼", Some(PosTag::NCN));
        assert_eq!(trie.total, 6);

        trie.insert("역도", Some(PosTag::NCN));
        assert_eq!(trie.total, 8);

        trie.insert("역무원", Some(PosTag::NCN));
        assert_eq!(trie.total, 13);

        trie.insert("역도산", Some(PosTag::NCN));
        assert_eq!(trie.total, 16);
    }
}
