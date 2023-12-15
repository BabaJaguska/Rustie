use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use std::fs::File;
use std::io::{self, BufReader};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub fn answer_question() -> anyhow::Result<()> {
    // Load a pre-trained model and tokenizer
    let qa_model = QuestionAnsweringModel::new(Default::default())?;

    // Define your question and context
    let question = String::from("What is the role of TP53 in cancer?");
    let context = String::from("TP53, also known as p53, is a gene that codes for a protein that regulates the cell cycle and hence functions as a tumor suppressor.");

    // Get the answer
    let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);

    println!("{:#?}", answers[0][0]);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(rename="CONTEXTS")]
    contexts: Vec<String>,

    #[serde(rename="LABELS")]
    labels: Option<Vec<String>>,

    #[serde(rename="LONG_ANSWER")]
    long_answer: Option<String>,

    #[serde(rename="MESHES")]
    meshes: Option<Vec<String>>,

    #[serde(rename="QUESTION")]
    question: String,

    #[serde(rename="YEAR")]
    year: Option<String>,

    final_decision: String,
    reasoning_free_pred: Option<String>,
    reasoning_required_pred: Option<String>,
}

type JsonData = HashMap<String, Entry>;

pub fn read_json(filepath: &str) -> io::Result<JsonData>{
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let json_stuff: JsonData = serde_json::from_reader(reader)?;
    Ok(json_stuff)    
}




