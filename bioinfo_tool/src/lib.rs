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

#[cfg(test)]
mod tests {
    use super::*;
    mod counting{
        use super::*;

        struct TestCase {
            dna: &'static str,
            expected_counts: HashMap<char, usize>,
        }

        #[test]
        fn test_count_nucleotides() {
            let test_cases = vec![
                TestCase {
                    dna: "AACCGGTA",
                    expected_counts: [('A', 3), ('C', 2), ('G', 2), ('T', 1)]
                        .iter()
                        .cloned()
                        .collect(),
                },
                TestCase {
                    dna: "",
                    expected_counts: HashMap::new(), // empty map for empty string
                },
                TestCase {
                    dna: "C",
                    expected_counts: [('C', 1)].iter().cloned().collect(),
                },
                TestCase {
                    dna: "TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT",
                    expected_counts: [('T', 40)].iter().cloned().collect(),
                },
            ];

            for test in test_cases {
                let counts = count_nucleotides(test.dna);
                assert_eq!(counts, test.expected_counts);
            }
        }
    }

    mod complementing{
        use super::*;
        struct TestCase {
            dna: &'static str,
            complement: &'static str,
        }

        #[test]
        fn test_complements() {
            let test_cases = vec![
                TestCase {
                    dna: "",
                    complement: ""
                },
                TestCase {
                    dna: "AACTGGA",
                    complement: "TTGACCT"
                }
            ];
            for test in test_cases{
                let complement = dna_complement(test.dna);
                assert_eq!(complement, test.complement);
            }
        }
    }
}
