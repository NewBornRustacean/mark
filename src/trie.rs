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

    pub fn insert(&mut self, word: &str, tag: Option<PosTag>) {
        let mut current_node = &mut self.root;

        for single_char in word.chars() {
            // iterate over word to deal with each of syllables
            match is_hangul(single_char) {
                true => {
                    // 한글은 초성, 중성, 종성 나눠서 insert
                    let (initial_consonant, mid_vowel, final_consonant) =
                        split_syllable(single_char);
                    current_node = current_node.children.entry(initial_consonant).or_default();
                    current_node = current_node.children.entry(mid_vowel).or_default();

                    if final_consonant != '\0' {
                        // 종성은 None이 아닌 경우에만 insert
                        current_node = current_node.children.entry(final_consonant).or_default();
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
    pub fn search() {}
}
