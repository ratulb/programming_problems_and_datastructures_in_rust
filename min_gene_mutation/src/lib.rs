///433. Minimum Genetic Mutation
///https://leetcode.com/problems/minimum-genetic-mutation/
///
///A gene string can be represented by an 8-character long string, with choices from 
///'A', 'C', 'G', and 'T'.

///Suppose we need to investigate a mutation from a gene string startGene to a gene 
///string endGene where one mutation is defined as one single character changed in 
///the gene string.

///For example, "AACCGGTT" --> "AACCGGTA" is one mutation.
///There is also a gene bank bank that records all the valid gene mutations. A gene 
///must be in bank to make it a valid gene string.

///Given the two gene strings startGene and endGene and the gene bank bank, return 
///the minimum number of mutations needed to mutate from startGene to endGene. If 
///there is no such a mutation, return -1.

///Note that the starting point is assumed to be valid, so it might not be included 
///in the bank.
///
///
use std::collections::{HashSet, VecDeque};
pub struct Solution;

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let bank = bank.into_iter().collect::<HashSet<_>>();
        if !bank.contains(&end_gene) {
            return -1;
        }
        if start_gene == end_gene {
          return 0;
        }
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        let gene_chars = vec!['A', 'C', 'G', 'T'];
        queue.push_back((start_gene.clone(), 0));
        seen.insert(start_gene);
        while let Some((gene, level)) = queue.pop_front() {
            if *gene == end_gene {
                return level;
            }
            for i in 0..gene.len() {
                for c in &gene_chars {
                    let mut new_gene = String::from(&gene[..i]);
                    new_gene.push(*c);
                    new_gene.push_str(&gene[i + 1..]);
                    if new_gene == end_gene && bank.contains(&new_gene) {
                        return level + 1;
                    }
                    if !seen.contains(&new_gene) && bank.contains(&new_gene) {
                        queue.push_back((new_gene.clone(), level + 1));
                        seen.insert(new_gene);
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_mutation_test_1() {
        let result = Solution::min_mutation(
            "AACCGGTT".to_string(),
            "AACCGGTA".to_string(),
            vec!["AACCGGTA".to_string()],
        );
        assert_eq!(result, 1);

        let result = Solution::min_mutation(
            "AACCGGTT".to_string(),
            "AAACGGTA".to_string(),
            vec![
                "AACCGGTA".to_string(),
                "AACCGCTA".to_string(),
                "AAACGGTA".to_string(),
            ],
        );
        assert_eq!(result, 2);
    }
}
