//! trie structure and its functions
//!
//!
use std::collections::HashMap;

use crate::hangul::{is_hangul, split_syllable, PosTag};

#[derive(Default, Debug)]
struct TrieNode {
    character: char,
    is_end: bool,
    children: HashMap<char, TrieNode>,
    tag: Option<PosTag>, // this field has a value if and only if is_end:=True
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

    fn create_node(&mut self, character: char, is_end: bool) -> TrieNode {
        let new_node = TrieNode {
            character: character,
            is_end: is_end,
            children: HashMap::default(),
            tag: None,
        };

        // self.total += 1;
        return new_node;
    }

    pub fn insert(&mut self, word: &str, tag: Option<PosTag>) {
        let mut current_node: &mut TrieNode = &mut self.root;

        for single_char in word.chars() {
            // iterate over word to deal with each of syllables
            match is_hangul(single_char) {
                true => {
                    // 한글은 초성, 중성, 종성 나눠서 insert
                    let (_initial_consonant, _mid_vowel, _final_consonant) =
                        split_syllable(single_char);

                    let initial_consonant = &_initial_consonant;
                    let mid_vowel = &_mid_vowel;
                    let final_consonant = &_final_consonant;

                    current_node = current_node
                        .children
                        .entry(*initial_consonant)
                        .or_insert_with(|| self.create_node(*initial_consonant, false));

                    current_node = current_node
                        .children
                        .entry(*mid_vowel)
                        .or_insert_with(|| self.create_node(*mid_vowel, false));

                    if final_consonant.ne(&'\0') {
                        // 종성은 None이 아닌 경우에만 insert
                        current_node = current_node
                            .children
                            .entry(*final_consonant)
                            .or_insert_with(|| self.create_node(*final_consonant, false));
                    }
                }
                false => {
                    // 한글이 아닌 문자는 그냥 insert
                    current_node = current_node.children.entry(single_char).or_default();
                }
            }
        }

        current_node.is_end = true;
        if tag.is_none() == false {
            current_node.tag = tag;
        }
    }

    /// this function is for the insert function.
    /// use "contains" function decripted below to search over trie.
    fn naive_search(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for single_char in word.chars() {
            match current_node.children.get(&single_char) {
                Some(node) => current_node = node,
                None => return false,
            }
        }

        current_node.is_end
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

        trie.insert("역삼", Some(PosTag::NNP));
        assert_eq!(trie.total, 6);

        trie.insert("역", Some(PosTag::NNG));
        assert_eq!(trie.total, 6);
    }
}
