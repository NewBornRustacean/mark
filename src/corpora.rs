/// utils for corpora.
/// parsing corpus from .json, get some stats, probs(e.g transition prob. for morpheme)
///
///
///
use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Morpheme {
    id: u8,
    form: String,
    label: String,
    word_id: u8,
    position: u8,
}

#[derive(Deserialize, Debug)]
struct Word {
    id: u8,
    form: String,
    begin: u8,
    end: u8,
}

#[derive(Deserialize, Debug)]
struct Sentence {
    id: String,
    form: String,
    words: Vec<Word>,
    morphemes: Vec<Morpheme>,
}

#[derive(Deserialize, Debug)]
pub struct Document {
    id: String,
    metadata: Option<String>,
    sentences: Vec<Sentence>,
}

// pub fn read_corpus_from_file<P: AsRef<Path>>(path: P) -> Result<Document, Box<dyn Error>> {
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);

//     let document = serde_json::from_reader(reader)?;

//     Ok(document)
// }
