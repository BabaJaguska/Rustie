fn main() {
    println!("Hello, world!");
    let dna: &str = "AAACCTTGGGGTAC";
    let counts = bioinfo_tool::count_nucleotides(dna);
    let complement = bioinfo_tool::dna_complement(dna);
    println!(
        "DNA: {}\nComplement: {}\nCounts: {:#?} ",
        dna, complement, counts
    );
}
