use std::collections::HashMap;

pub fn count_nucleotides(dna: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for nucleotide in dna.chars() {
        *counts.entry(nucleotide).or_insert(0) += 1;
    }
    counts
}

pub fn dna_complement(dna: &str) -> String {
    dna.chars()
        .map(|nucleotide| {
            match nucleotide {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => nucleotide, // Handle unexpected characters gracefully
            }
        })
        .collect()
}
