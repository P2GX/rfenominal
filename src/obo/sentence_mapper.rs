//! SentanceMapper
//! 
//! Structure to coordinate the text mining of an individual sentence.
//! An assumption of this application is that any valid HPO term will be 
//! completely contained within a sentence. Therefore, fenominal first
//! splits the input text into sentences, and then performance text mining
//! on each sentence in this module.

use std::cmp::min;
use std::marker::PhantomData;
use std::sync::Arc;
use once_cell::sync::Lazy;
use ontolius::ontology::{HierarchyWalks, OntologyTerms};
use ontolius::term::{MinimalTerm, Synonymous};
use std::collections::HashSet;
use crate::models::fenominal_model::FenominalHit;
use crate::models::ontology_profile::OntologyProfile;
use crate::stopwords::is_stop;
use crate::util::error::FenominalError;
use crate::util::negex::NegEx;
use crate::{simple_sentence::SimpleSentence, simple_token::SimpleToken};
use crate::obo::term_mapper::TermMapper;

/// This is a set of words that we use to indentify exclusion (negation) of phenotypic abnormality
///
/// e.g. "Proband 1 did not have arachnodactyly" would be flagged as negated because of the word "not".
static NEGATION_CLUES: Lazy<HashSet<String>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("no".to_string());
    set.insert("nil".to_string());
    set.insert("denies".to_string());
    set.insert("not".to_string());
    set.insert("exclude".to_string());
    set.insert("excluded".to_string());
    set.insert("screen".to_string());
    set.insert("screening".to_string());
    set.insert("normal".to_string());
    set
});

pub struct SentenceMapper<O, T> where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous {
    term_mapper: TermMapper,
    ontology: Arc<O>,
    _marker: PhantomData<T>,
    negex: NegEx,
}

impl<O, T>  SentenceMapper<O, T> where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous {
     pub fn new(ontology: Arc<O>, profile: &OntologyProfile) -> Self {
        let mapper = TermMapper::new(ontology.clone(), profile);
        SentenceMapper { 
            term_mapper: mapper,
            ontology: ontology.clone(),
            _marker: PhantomData,
            negex: NegEx::from_embedded(),
        }
    }

    pub fn map_sentence(&self, simple_sentence: &SimpleSentence) -> Result<Vec<FenominalHit>, FenominalError> {
        let full_sentence_refs: Vec<&str> = simple_sentence.get_tokens()
            .iter()
            .map(|t| t.get_lc_original_token())
            .collect();
        let tokens: &[SimpleToken] = simple_sentence.get_tokens();
        // remove stop words from tokens
        let nonstop_tokens: Vec<&SimpleToken> = tokens
            .iter()
            .filter(|tk| !is_stop(tk.get_original_token()))
            .collect();
        let start_pos_offset = simple_sentence.get_start_pos();
        let mut mapped_sentence_part_list = Vec::new();
        // Check window sizes from largest to smallest
        let max_window = min(self.term_mapper.max_tokens(), tokens.len());
        // was the corresponding token already used for a "hit"?
        let mut token_used = vec![false; nonstop_tokens.len()];
        for window_size in (1..=max_window).rev() {
            // .windows(n) slides 1 token at a time: [0,1,2], [1,2,3], [2,3,4]...
            for (idx, chunks) in nonstop_tokens.windows(window_size).enumerate() {
                // Skip this window if ANY token in it is already part of a longer match
                if token_used[idx..idx + window_size].iter().any(|&used| used) {
                    continue;
                }
                let string_chunk_refs: Vec<&str> = chunks
                    .iter()
                    .map(|stoken| stoken.get_lc_original_token())
                    .collect();
                if let Some(term_match) = self.term_mapper.get_match(&string_chunk_refs) {
                    let term_id = term_match.get_term_id();
                    let term = self.ontology.term_by_id(term_id)
                        .ok_or_else(|| FenominalError::term_retrieval_error(term_id))?;
                    // Get character positions from the tokens
                    let start_char = chunks[0].get_start_pos() + start_pos_offset;
                    let end_char = chunks[chunks.len() - 1].get_end_pos() + start_pos_offset;
                    // The range relative to the FULL original sentence
                    let first_token_idx = chunks[0].index;
                    let last_token_idx = chunks[chunks.len() - 1].index;
                    let hit_idx_range = first_token_idx..(last_token_idx + 1);
                    let is_excluded = self.negex.is_negated(&full_sentence_refs, hit_idx_range);

                    let hit = FenominalHit::new(
                        term_id.to_string(),
                        term.name(),
                        start_char..end_char,
                        !is_excluded,
                    );
                    for i in idx..idx + window_size {
                        token_used[i] = true;
                    }
                    mapped_sentence_part_list.push(hit);
                } 
            }
        }
        // Sort according to order of appearance
        mapped_sentence_part_list.sort_by_key(|h| h.span.start);
        Ok(mapped_sentence_part_list)
    }

    fn has_negation(&self, tokens: &[SimpleToken]) -> bool {
        tokens
            .iter()
            .any(|token| NEGATION_CLUES.contains(token.get_lc_original_token()))
    }
}


// region:    --- Tests

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, str::FromStr};

    use ontolius::TermId;
    use rstest::{fixture, rstest};

    use crate::obo::concept::Concept;

    use super::*;

    
#[fixture]
pub fn paramedian_cleft_palate() -> Concept {
    let hpo_id = TermId::from_str("HP:0009099").unwrap();
    let label = "paramedian cleft lip";
    Concept::new(label, hpo_id)
} 

#[fixture]
fn decreased_hc() -> Concept {
    // Microcephaly HP:0000252
    let hpo_id = TermId::from_str("HP:0040195").unwrap();
    let label = "Decreased head circumference";
    Concept::new(label, hpo_id)
}

#[fixture]
fn component_token_to_concept_map(
    decreased_hc: Concept,
    paramedian_cleft_palate: Concept
) -> HashMap<String, Vec<Concept>> {
    let mut map: HashMap<String, Vec<Concept>> = HashMap::new();
    let dch = vec![decreased_hc];
    for token in vec!["Decreased", "head", "circumference"] {
        map.insert(token.to_string(), dch.clone());
    };
    let pcp = vec![paramedian_cleft_palate];
    for token in vec!["paramedian", "cleft", "lip"] {
        map.insert(token.to_string(), pcp.clone());
    };
    map
}



#[rstest]
fn paramedian_cp(
    component_token_to_concept_map:HashMap<String, Vec<Concept>>,
    paramedian_cleft_palate: Concept
)  {
    let result = component_token_to_concept_map.get("cleft");
    assert!(result.is_some());
    let hpo_concept_list = result.unwrap();
    assert_eq!(1, hpo_concept_list.len());
    let hpo_concept = hpo_concept_list[0].clone();
    let expected_term_id: &TermId = paramedian_cleft_palate.get_term_id();
    let observed_term_id: &TermId = hpo_concept.get_term_id();
    assert_eq!(expected_term_id, observed_term_id);
}


    
}

// endregion: --- Tests