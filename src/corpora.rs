use std::fs::File;
use std::io::BufReader;

use kdam::{tqdm, BarExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct Morpheme{
    pub term: String,
    pub tag: String,
    pub category: Option<String>,
}

/// Reads a JSON file containing morphemes and returns a vector of `Morpheme` structs.
///
/// # Arguments
///
/// * `source_path` - A string slice that holds the path of the JSON file.
///
/// # Returns
///
/// * A `Result` containing either a vector of `Morpheme` structs or an `Error`.
///
/// # Errors
///
/// * If the file cannot be opened, an `Error` is returned.
/// * If the file cannot be parsed as a valid JSON, an `Error` is returned.
///
/// # Examples
///
/// ```
/// let morphs = read_nia_morphs("morphs.json").unwrap();
/// println!("{:?}", morphs);
/// ```
pub fn read_nia_morphs(source_path: &str) -> Result<Vec<Morpheme>, Error> {
    let file = File::open(source_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let json_data: Vec<Morpheme> = serde_json::from_reader(reader).unwrap();
    Ok(json_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_nia_morphs() {
        let dir = tempdir().unwrap();
        let source_path = dir.path().join("morphs.json");
        let mut file = File::create(&source_path).unwrap();
        let data = r#"[{"term": "나는", "tag": "Noun", "category": ""}, {"term": "밥을", "tag": "Noun", "category": "Object"}, {"term": "먹었다", "tag": "Verb", "category": "Predicate"}]"#;
        file.write_all(data.as_bytes()).unwrap();

        let result = read_nia_morphs(source_path.to_str().unwrap()).unwrap();

        // assert that the result is a vector of three morphemes
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].term, "나는");
        assert_eq!(result[0].tag, "Noun");
        assert_eq!(result[0].category.clone().unwrap(), "");
        assert_eq!(result[1].term, "밥을");
        assert_eq!(result[1].tag, "Noun");
        assert_eq!(result[1].category.clone().unwrap(), "Object");
        assert_eq!(result[2].term, "먹었다");
        assert_eq!(result[2].tag, "Verb");
        assert_eq!(result[2].category.clone().unwrap(), "Predicate");
    }
}
