/// utils for corpora.
/// parsing corpus from .json, get some stats, probs(e.g transition prob. for morpheme)
///
///
///
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use kdam::{tqdm, BarExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};

/// Def:
///   형태소를 저장하기 위한 구조체.
/// Note:
///   - id, position은 해당 형태소의 출처 문장에서 정의되므로 유일하지 않음(=형태소의 식별자로 사용될 수 없음).
///   - 따라서, uniq_morphs.json을 만들때는 "form" 과 "label"을 복합키로 사용함.
#[derive(Deserialize, Serialize, Debug)]
pub struct Morpheme {
    id: u8,        //1부터 시작해서 1씩 증가.
    form: String,  //형태소 문자.
    label: String, //형태소 태그
    word_id: u8, // 형태소가 등장한 단어의 id. 문장내에서 정의되는 식별자이므로 전역(global) 아이디로 사용할 수 없음.
    position: u8, // 형태소의 문장 내 위치.
}

/// Def:
///   유일한 형태소만 저장된 json파일 읽어오는 함수.
/// Note:
///   - SXMP = 32402 개
pub fn read_uniq_morphs(source_path: &str) -> Result<Vec<Morpheme>, Error> {
    let file = File::open(source_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let json_data: Vec<Morpheme> = serde_json::from_reader(reader).unwrap();
    Ok(json_data)
}

/// Def:
///    유일한 형태소를 json으로 저장
pub fn save_morphemes_to_json(
    morphemes: &Vec<Morpheme>,
    target_path: &str,
) -> Result<(), serde_json::Error> {
    let file = File::create(target_path).unwrap();

    serde_json::to_writer(file, morphemes)?;
    Ok(())
}

/// Def:
///   모두의 말뭉치 파일 읽어서 형태소만 중복 제거된 상태로 반환.
/// Note:
///   - 중복 제거 기준은 <form, label>을 pair key로.
///   - 이 함수는 중복 제거를 위해 한 번 사용하고, 이후에는 uniq_morps.json을 읽어오면 됨.
pub fn make_morphemes_unique(file_path: &str) -> Result<Vec<Morpheme>, Error> {
    let file = File::open(file_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader).unwrap();
    let document: &Vec<Value> = json_data["document"].as_array().unwrap();
    if document.is_empty() | (document.len() == 0) {
        panic!("document is empty!");
    } else {
        return Ok(get_uniq_morphemes_from_value(document));
    }
}

fn get_uniq_morphemes_from_value(document: &Vec<Value>) -> Vec<Morpheme> {
    let mut uniq_morph_form_label: HashSet<(String, String)> = HashSet::new();
    let mut pb = tqdm!(total = 100);
    let mut uniq_morphemes: Vec<Morpheme> = Vec::new();
    let json_null = vec![json!(null)];

    for doc in document {
        let sentences: &Vec<Value> = doc["sentence"].as_array().unwrap_or(&json_null);
        if sentences.is_empty() | (sentences.len() == 0) {
            continue;
        }

        for sent in sentences {
            let morphemes: &Vec<Value> = sent["morpheme"].as_array().unwrap_or(&json_null);
            if morphemes.is_empty() | (morphemes.len() == 0) {
                continue;
            }
            for morph in morphemes {
                if morph.is_null() {
                    continue;
                }
                let morph_typed: Morpheme = serde_json::from_value(morph.clone()).unwrap();
                let check_pair = (morph_typed.form.clone(), morph_typed.label.clone());

                if !uniq_morph_form_label.contains(&check_pair) {
                    uniq_morph_form_label.insert(check_pair);
                    uniq_morphemes.push(morph_typed);
                }
            }
        }
        pb.update(1);
    }
    return uniq_morphemes;
}

#[cfg(test)]
mod test_corpora {
    use super::*;

    #[test]
    fn test_read_corpus_from_file() {
        let data = r#"
        {
            "id": "NXMP1902008040",
            "metadata": {},
            "document": [
              {
                "id": "NWRW1800000022.417",
                "metadata": {},
                "sentence": [
                  {
                    "id": "NWRW1800000022.417.1.1",
                    "form": "[제주·서울] \"세계환경수도 조성위해 10개년 실천계획 만들겠다\" 김태환 지사 밝혀",
                    "word": [
                      {
                        "id": 1,
                        "form": "[제주·서울]",
                        "begin": 0,
                        "end": 7
                      },
                      {
                        "id": 2,
                        "form": "\"세계환경수도",
                        "begin": 8,
                        "end": 15
                      },
                      {
                        "id": 3,
                        "form": "조성위해",
                        "begin": 16,
                        "end": 20
                      }
                    ],
                    "morpheme": [
                      {
                        "id": 1,
                        "form": "[",
                        "label": "SS",
                        "word_id": 1,
                        "position": 1
                      },
                      {
                        "id": 2,
                        "form": "제주",
                        "label": "NNP",
                        "word_id": 1,
                        "position": 2
                      },
                      {
                        "id": 3,
                        "form": "·",
                        "label": "SP",
                        "word_id": 1,
                        "position": 3
                      }
                    ],
                    "WSD": []
                  }
                ]
              }
            ]
          }"#;
        let json_data: Value = serde_json::from_str(data).unwrap();
        let document: &Vec<Value> = json_data["document"].as_array().unwrap();
        let morphs = get_uniq_morphemes_from_value(document);
        println!("{:?}", morphs[0]);
    }
}
