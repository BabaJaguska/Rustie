use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};


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