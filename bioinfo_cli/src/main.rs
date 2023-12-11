use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Bioinfo_Tool")
        .version("1.0")
        .author("BabaJaguska")
        .about("Performs basic operations on a genomic sequence.")
        .arg(
            Arg::with_name("sequence")
                .short('s')
                .long("sequence")
                .value_name("SEQUENCE")
                .help("DNA sequence to process")
                .required(true)
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("count").about("Counts nucleotides in the sequence"))
        .subcommand(SubCommand::with_name("complement").about("Generates complementary DNA strain"))
        .get_matches();

    let sequence = matches.value_of("sequence").unwrap();

    match matches.subcommand_name() {
        Some("count") => {
            let counts = bioinfo_tool::count_nucleotides(sequence);
            println!("Nucleotide counts:\n{:?}", counts)
        }
        Some("complement") => {
            let complement = bioinfo_tool::dna_complement(sequence);
            println!("Complement:\n{}", complement);
        }
        Some(_) => {
            println!("Invalid command");
        }
        None => {
            println!("Please choose what to do.");
        }
    }
}
