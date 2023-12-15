
fn main(){
    //let _ = bioinfo_qa::answer_question();
    let datafile: &str = "./bioinfo_qa/pubmed_data.json";
    let data = bioinfo_qa::read_json(datafile);

    match data {
        Ok(json_data) =>{
            // Print a random entry
            println!("{:#?}", json_data.iter().next());
        }
        Err(e) => {
            println!("Sorry bro, {}", e);
        }
    }
    
}
